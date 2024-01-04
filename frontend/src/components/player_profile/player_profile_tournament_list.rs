use gloo_net::http::Request;
use js_sys::encode_uri_component;
use web_sys::{window, HtmlElement};
use yew::{function_component, html, use_effect_with, use_state, Callback, Html, Properties};

use crate::models::{CaptchaRequest, ImageData, Set, Tournament, UploadResponse};

use crate::components::loading_spinner::LoadingSpinner;
use crate::utils::{calculate_spr_or_uf, create_page_numbers};
use wasm_bindgen::prelude::*;
use yew_recaptcha_v3::recaptcha::use_recaptcha;

const RECAPTCHA_SITE_KEY: &str = std::env!("RECAPTCHA_SITE_KEY");
const TOURNAMENT_PAGE_SIZE: usize = 5;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub display: bool,
    pub selected_player_tournaments: Option<Vec<Tournament>>,
    pub selected_tournament_sets: Option<Vec<Set>>,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    pub fn html2canvas(element: &HtmlElement) -> js_sys::Promise;
}

#[function_component(PlayerProfileTournamentList)]
pub fn player_profile_tournament_list(props: &Props) -> Html {
    let tournament_list_current_page = use_state(|| 1);
    let tournament_list_total_pages = use_state(|| 0);
    let tournament_list_start_index = use_state(|| 0);
    let tournament_list_end_index = use_state(|| 0);

    let paginated_tournaments = use_state(Vec::<Tournament>::new);

    // Use effect that updates pagination state when selected_player_tournaments changes
    {
        let selected_player_tournaments = props.selected_player_tournaments.clone();
        let tournament_list_total_pages = tournament_list_total_pages.clone();
        let tournament_list_end_index = tournament_list_end_index.clone();
        let paginated_tournaments = paginated_tournaments.clone();

        use_effect_with(props.selected_player_tournaments.clone(), move |_| {
            // Calculate total pages
            let total_pages = selected_player_tournaments
                .as_ref()
                .map_or(0, |tournaments| {
                    (tournaments.len() as f32 / TOURNAMENT_PAGE_SIZE as f32).ceil() as usize
                });

            // Update total pages state
            tournament_list_total_pages.set(total_pages);

            // Calculate paginated tournaments
            if let Some(tournaments) = selected_player_tournaments.as_ref() {
                let end = usize::min(TOURNAMENT_PAGE_SIZE, tournaments.len());
                tournament_list_end_index.set(end);
                paginated_tournaments.set(tournaments[0..end].to_vec());
            }
        });
    }

    {
        let tournament_list_start_index = tournament_list_start_index.clone();
        let tournament_list_end_index = tournament_list_end_index.clone();
        let paginated_tournaments = paginated_tournaments.clone();
        let tournament_list_current_page = tournament_list_current_page.clone();
        let selected_player_tournaments = props.selected_player_tournaments.clone();

        use_effect_with(tournament_list_current_page.clone(), move |_| {
            if let Some(spt) = selected_player_tournaments {
                let start = (*tournament_list_current_page - 1) * TOURNAMENT_PAGE_SIZE;
                let end = usize::min(
                    start + TOURNAMENT_PAGE_SIZE,
                    spt.len(),
                );

                tournament_list_start_index.set(start);
                tournament_list_end_index.set(end);

                let spt = spt.clone();
                let paginated_tournaments = paginated_tournaments.clone();

                paginated_tournaments.set(spt[start..end].to_vec());
            }
        });
    }

    let tournament_list_curr_page = (*tournament_list_current_page).clone();
    let tournament_list_tot_pages = (*tournament_list_total_pages).clone();

    let tournament_list_pagination_numbers =
        create_page_numbers(tournament_list_curr_page, tournament_list_tot_pages);

    let is_screenshotting = use_state(|| None::<i32>);

    let last_token = use_state(|| None);
    let on_execute = use_state(|| None);

    // Recaptcha will be called only when on_execute changes.
    let on_execute_clone = on_execute.clone();
    use_recaptcha(RECAPTCHA_SITE_KEY.to_string(), on_execute_clone);

    {
        let is_screenshotting = is_screenshotting.clone();
        let last_token = last_token.clone();
        use_effect_with(last_token, move |lt| {
            if (*is_screenshotting).is_some() {
                let window = window().unwrap();
                let document = window.document().unwrap();
                let screenshotting_id = (*is_screenshotting).unwrap();
                let element = document
                    .get_element_by_id(&format!("result-section-{}", screenshotting_id))
                    .unwrap();
                let html_element = element.dyn_into::<HtmlElement>().unwrap();

                let promise = html2canvas(&html_element);

                let is_screenshotting = is_screenshotting.clone();
                let lt = lt.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let captcha_res =
                        Request::post(&format!("{}/check-captcha", env!("SERVER_ADDRESS_2")))
                            .json(&CaptchaRequest {
                                token: (*lt).clone().unwrap(),
                            })
                            .unwrap()
                            .send()
                            .await;

                    match captcha_res {
                        Ok(response) if response.ok() => {
                            let result = wasm_bindgen_futures::JsFuture::from(promise).await;
                            match result {
                                Ok(canvas) => {
                                    let canvas: web_sys::HtmlCanvasElement =
                                        canvas.dyn_into().unwrap_throw();
                                    let data_url = canvas.to_data_url().unwrap_throw();

                                    let upload_res = Request::post(&format!(
                                        "{}/upload",
                                        env!("SERVER_ADDRESS_2")
                                    ))
                                    .header("Content-Type", "application/json")
                                    .json(&ImageData { image: data_url })
                                    .unwrap()
                                    .send()
                                    .await;

                                    if let Ok(upload_response) = upload_res {
                                        if upload_response.ok() {
                                            let json_result: Result<UploadResponse, _> =
                                                upload_response.json().await;
                                            match json_result {
                                                Ok(UploadResponse::Success(success)) => {
                                                    // Construct Twitter Web Intent URL
                                                    let twitter_message = "Heads up, the URL below will be rendered as an image once you send out the tweet - feel free to delete this message and add your own comment about your run while leaving the URL at the bottom.\n";
                                                    let image_url = format!(
                                                        "https://smithe.pictures/image/{}",
                                                        success.filename
                                                    );
                                                    let tweet_intent_url = format!("https://twitter.com/intent/tweet?text={}%0A{}", encode_uri_component(twitter_message), encode_uri_component(&image_url));

                                                    // Open the Twitter Intent URL in a new tab/window
                                                    let window = web_sys::window().unwrap();
                                                    let _ = window.open_with_url(&tweet_intent_url);
                                                }
                                                Ok(UploadResponse::Error(error)) => {
                                                    web_sys::console::log_1(
                                                        &format!("Error: {}", error.message).into(),
                                                    );
                                                }
                                                Err(e) => {
                                                    web_sys::console::log_1(
                                                        &format!(
                                                            "Failed to parse JSON response: {:?}",
                                                            e
                                                        )
                                                        .into(),
                                                    );
                                                }
                                            }
                                        } else {
                                            web_sys::console::log_1(
                                                &"Image upload failed with non-success status"
                                                    .into(),
                                            );
                                        }
                                    } else {
                                        web_sys::console::log_1(&"Failed to send image".into());
                                    }
                                }
                                Err(e) => {
                                    web_sys::console::log_1(&format!("Error: {:?}", e).into())
                                }
                            }
                        }
                        _ => {
                            // Handle captcha verification failure
                            web_sys::console::log_1(&"Captcha verification failed".into());
                        }
                    }

                    is_screenshotting.set(None);
                });
            }
        });
    }

    if props.display {
        html! {
            html! {
                <div>
                    <div class="accordion" id="accordion">
                    {
                        paginated_tournaments.iter().map(|t| {
                            let onclick = {
                                let last_token = last_token.clone();
                                let on_execute = on_execute.clone();
                                let is_screenshotting = is_screenshotting.clone();
                                let tid = t.tournament_id;
                                Callback::from(move |_| {
                                    let last_token = last_token.clone();
                                    // setting the on_execute callback will force recaptcha to be recalculated.
                                    on_execute.set(Some(Callback::from(move |token| {
                                        last_token.set(Some(token));
                                    })));

                                    is_screenshotting.set(Some(tid));
                                    ()
                                })
                            };
                            html! {
                                <div class="accordion-item">
                                    <h2 class="accordion-header" id={format!("heading-{}", t.tournament_id)}>
                                        <button class="accordion-button collapsed" type="button" data-bs-toggle="collapse" data-bs-target={format!("#collapse-{}", t.tournament_id)} aria-expanded="false" aria-controls={format!("collapse-{}", t.tournament_id)}>
                                            <div class="col-md-10">
                                                <h4 class="fw-bold">{(&t.event_name).to_string()}</h4>
                                                <hr/>
                                                <div class="row">
                                                    <div class="col-md-2 col-10">
                                                        <div class="card border-0 text-center">
                                                            <div class="card-body">
                                                                <h5 class="card-title fw-bold">{"Seed"}</h5>
                                                                <p class="card-text">{&t.seed}</p>
                                                            </div>
                                                        </div>
                                                    </div>

                                                    <div class="col-md-2 col-10">
                                                        <div class="card border-0 text-center">
                                                            <div class="card-body">
                                                                <h5 class="card-title fw-bold">{"Placement"}</h5>
                                                                <p class="card-text">{format!("{}/{}", &t.placement, &t.num_entrants)}</p>
                                                            </div>
                                                        </div>
                                                    </div>

                                                    <div class="col-md-2 col-10">
                                                        <div class="card border-0 text-center">
                                                            <div class="card-body">
                                                                <h5 class="card-title fw-bold">{"SPR"}</h5>
                                                                <p class="card-text">{format!("{}", calculate_spr_or_uf(t.seed, t.placement))}</p>
                                                            </div>
                                                        </div>
                                                    </div>
                                                </div>
                                                <br/>
                                            </div>
                                        </button>
                                    </h2>
                                    <div id={format!("collapse-{}", t.tournament_id)} class="accordion-collapse collapse" aria-labelledby={format!("heading-{}", t.tournament_id)} data-bs-parent="#accordion">
                                        <div class="accordion-body">
                                            <ul class="list-group">
                                                {
                                                    props.selected_tournament_sets.as_ref().unwrap().iter().filter(|s| s.tournament_id == t.tournament_id).map(|s| {
                                                        html! {
                                                            <li class="list-group-item">
                                                            <div class="row">
                                                                <div class="col-md-4">
                                                                    {format!("{} - {} ", s.requester_score, s.opponent_score)}
                                                                    {
                                                                        if s.requester_score > s.opponent_score {
                                                                            html! {
                                                                                <strong class="text-success">{"WIN "}</strong>
                                                                            }
                                                                        } else if s.requester_score == s.opponent_score {
                                                                            html! {
                                                                                <strong class="text-warning">{"TIE "}</strong>
                                                                            }
                                                                        } else {
                                                                            html! {
                                                                                <strong class="text-danger">{"LOSS "}</strong>
                                                                            }
                                                                        }
                                                                    }
                                                                    {format!("against {} (seed: {})", s.opponent_tag_with_prefix, s.opponent_seed)}
                                                                </div>
                                                                <div class="col-md-4 offset-md-4">
                                                                    <div class="row justify-content-end p-1">
                                                                    {
                                                                        if s.requester_score > s.opponent_score
                                                                            && s.requester_seed > s.opponent_seed
                                                                            && s.result_type > 1
                                                                            && calculate_spr_or_uf(s.requester_seed, s.opponent_seed) > 0 {
                                                                            format!("upset factor: {}", calculate_spr_or_uf(s.requester_seed, s.opponent_seed))
                                                                        } else {
                                                                            "".to_string()
                                                                        }
                                                                    }
                                                                    </div>
                                                                </div>
                                                            </div>
                                                            </li>
                                                        }
                                                    }
                                                    ).collect::<Html>()
                                                }
                                                <div class="row justify-content-end p-2">
                                                    {
                                                        if (*is_screenshotting).is_none() {
                                                            html! {
                                                                <>
                                                                    <div class="col-auto">
                                                                        <a href={format!("{}", t.link)}
                                                                            target="_blank" rel="noopener noreferrer" class="btn btn-primary btn-sm">
                                                                            <i class="bi bi-trophy" aria-hidden="true"></i> {" View on StartGG"}
                                                                        </a>
                                                                    </div>
                                                                    <div class="col-auto">
                                                                        <a {onclick} rel="noopener noreferrer" class="btn btn-secondary btn-sm">
                                                                            <i class="bi bi-twitter" aria-hidden="true"></i> {" Share on Twitter"}
                                                                        </a>
                                                                    </div>
                                                                </>
                                                            }
                                                        } else {
                                                            html! {
                                                                <div class="screenshot-container" id={format!("result-section-{}", t.tournament_id)}>
                                                                    <div class="tournament-info">
                                                                        <h3 class="tournament-title">{(&t.event_name).to_string()}</h3>
                                                                        <p class="tournament-details">
                                                                            {format!("Seed: {}, Placement: {}/{}", &t.seed, &t.placement, &t.num_entrants)}
                                                                        </p>
                                                                    </div>
                                                                    <div class="match-results">
                                                                        {
                                                                            for props.selected_tournament_sets.as_ref().unwrap().iter().filter(|s| s.tournament_id == t.tournament_id).map(|s| {
                                                                                html! {
                                                                                    <div class="match-result">
                                                                                        <span class={if s.requester_score > s.opponent_score { "win" } else if s.requester_score < s.opponent_score { "loss" } else { "tie" }}>
                                                                                            {format!("{} - {} vs {} (Seed: {})", s.requester_score, s.opponent_score, s.opponent_tag_with_prefix, s.opponent_seed)}
                                                                                        </span>
                                                                                    </div>
                                                                                }
                                                                            })
                                                                        }
                                                                    </div>
                                                                    <div class="twitter-footer">
                                                                        <span class="screenshot-message">
                                                                            {format!("See my full results at smithe.net/player/{}", t.requester_id)}
                                                                        </span>
                                                                    </div>
                                                                </div>
                                                            }
                                                        }
                                                    }
                                                </div>
                                            </ul>
                                        </div>
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                    </div>
                    <br/>
                    <br/>
                    <nav>
                    <ul class="pagination justify-content-center">
                        <li class={if *tournament_list_current_page == 1 { "page-item disabled" } else { "page-item" }}>
                        <a class="page-link" href="#"
                            onclick={
                                let tournament_list_current_page = tournament_list_current_page.clone();
                                Callback::from(move |_| tournament_list_current_page.set(usize::max(1, *tournament_list_current_page - 1)))
                            }>{"Previous"}</a>
                        </li>
                        {
                            for tournament_list_pagination_numbers.iter().map(|&num| {
                                if num == 0 {
                                    html! { <li class="page-item disabled"><span class="page-link">{"..."}</span></li> }
                                } else {
                                    let is_active = num == *tournament_list_current_page;
                                    html! {
                                        <li class={if is_active { "page-item active" } else { "page-item" }}>
                                        <a class="page-link" href="#"
                                            onclick={
                                                let tournament_list_current_page = tournament_list_current_page.clone();
                                                Callback::from(move |_| tournament_list_current_page.set(num))}>
                                            { num.to_string() }
                                        </a>
                                        </li>
                                    }
                                }
                            })
                        }
                        <li class={if *tournament_list_current_page == *tournament_list_total_pages { "page-item disabled" } else { "page-item" }}>
                        <a class="page-link" href="#"
                            onclick={
                                let tournament_list_current_page = tournament_list_current_page.clone();
                                Callback::from(move |_| tournament_list_current_page.set(usize::min(*tournament_list_total_pages, *tournament_list_current_page + 1)))
                            }
                        >{"Next"}</a>
                        </li>
                    </ul>
                </nav>
                <br/>
                <br/>
                </div>
            }
        }
    } else {
        html! {
            <LoadingSpinner/>
        }
    }
}
