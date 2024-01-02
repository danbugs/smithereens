use web_sys::{window, HtmlElement};
use yew::{function_component, html, use_effect_with, use_state, Callback, Html, Properties};

use crate::models::{Set, Tournament};

use crate::components::loading_spinner::LoadingSpinner;
use crate::utils::calculate_spr_or_uf;
use wasm_bindgen::prelude::*;

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
    let is_screenshotting = use_state(|| None::<i32>);
    let image_data_url = use_state(|| None::<String>);

    {
        let image_data_url = image_data_url.clone();
        let is_screenshotting = is_screenshotting.clone();
        use_effect_with(is_screenshotting, move |is_screenshotting| {
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
                let image_data_url = image_data_url.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let result = wasm_bindgen_futures::JsFuture::from(promise).await;

                    match result {
                        Ok(canvas) => {
                            let canvas: web_sys::HtmlCanvasElement =
                                canvas.dyn_into().unwrap_throw();
                            let data_url = canvas.to_data_url().unwrap_throw();
                            
                            web_sys::console::log_1(&format!("{:?}", data_url).into());

                            image_data_url.set(Some(data_url));
                        }
                        Err(e) => web_sys::console::log_1(&format!("Error: {:?}", e).into()),
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
                        props.selected_player_tournaments.as_ref().unwrap().iter().map(|t| {
                            let onclick = {
                                let is_screenshotting = is_screenshotting.clone();
                                let tid = t.tournament_id;
                                Callback::from(move |_| {
                                    is_screenshotting.set(Some(tid));
                                })
                            };
                            html! {
                                <div class="accordion-item" id={format!("result-section-{}", t.tournament_id)}>
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
                                                                <div class="col-12 text-center">
                                                                    <span class="screenshot-message">
                                                                        {format!("See my full results at smithe.net/player/{}", t.requester_id)}
                                                                    </span>
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
                </div>
            }
        }
    } else {
        html! {
            <LoadingSpinner/>
        }
    }
}
