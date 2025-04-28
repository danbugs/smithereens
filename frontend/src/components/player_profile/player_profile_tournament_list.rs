use web_sys::HtmlElement;
use yew::{function_component, html, use_effect_with, use_state, Callback, Html, Properties};

use crate::models::{Set, Tournament};

use crate::components::loading_spinner::LoadingSpinner;
use crate::utils::{calculate_spr_or_uf, create_page_numbers};
use wasm_bindgen::prelude::*;

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
                let end = usize::min(start + TOURNAMENT_PAGE_SIZE, spt.len());

                tournament_list_start_index.set(start);
                tournament_list_end_index.set(end);

                paginated_tournaments.set(spt[start..end].to_vec());
            }
        });
    }

    let tournament_list_curr_page = *tournament_list_current_page;
    let tournament_list_tot_pages = *tournament_list_total_pages;

    let tournament_list_pagination_numbers =
        create_page_numbers(tournament_list_curr_page, tournament_list_tot_pages);

    if props.display {
        html! {
            html! {
                <div>
                    <div class="accordion" id="accordion">
                    {
                        paginated_tournaments.iter().map(|t| {
                            html! {
                                <div class="accordion-item">
                                    <h2 class="accordion-header" id={format!("heading-{}", t.tournament_id)}>
                                        <button class="accordion-button collapsed" type="button" data-bs-toggle="collapse" data-bs-target={format!("#collapse-{}", t.tournament_id)} aria-expanded="false" aria-controls={format!("collapse-{}", t.tournament_id)}>
                                            <div class="col-md-10">
                                                <h4 class="fw-bold">{t.event_name.to_string()}</h4>
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
                                                                        match s.requester_score.cmp(&s.opponent_score) {
                                                                            std::cmp::Ordering::Greater => html! {
                                                                                <strong class="text-success">{"WIN "}</strong>
                                                                            },
                                                                            std::cmp::Ordering::Equal => html! {
                                                                                <strong class="text-warning">{"TIE "}</strong>
                                                                            },
                                                                            std::cmp::Ordering::Less => html! {
                                                                                <strong class="text-danger">{"LOSS "}</strong>
                                                                            },
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
                                                    <div class="col-auto">
                                                        <a href={t.link.to_string()}
                                                            target="_blank" rel="noopener noreferrer" class="btn btn-primary btn-sm">
                                                            <i class="bi bi-trophy" aria-hidden="true"></i> {" View on StartGG"}
                                                        </a>
                                                    </div>
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
                        <button class="page-link" onclick={
                                let tournament_list_current_page = tournament_list_current_page.clone();
                                Callback::from(move |_| tournament_list_current_page.set(usize::max(1, *tournament_list_current_page - 1)))
                            }>{"Previous"}</button>
                        </li>
                        {
                            for tournament_list_pagination_numbers.iter().map(|&num| {
                                if num == 0 {
                                    html! { <li class="page-item disabled"><span class="page-link">{"..."}</span></li> }
                                } else {
                                    let is_active = num == *tournament_list_current_page;
                                    html! {
                                        <li class={if is_active { "page-item active" } else { "page-item" }}>
                                        <button class="page-link" onclick={
                                                let tournament_list_current_page = tournament_list_current_page.clone();
                                                Callback::from(move |_| tournament_list_current_page.set(num))}>
                                            { num.to_string() }
                                        </button>
                                        </li>
                                    }
                                }
                            })
                        }
                        <li class={if *tournament_list_current_page == *tournament_list_total_pages { "page-item disabled" } else { "page-item" }}>
                        <button class="page-link" onclick={
                                let tournament_list_current_page = tournament_list_current_page.clone();
                                Callback::from(move |_| tournament_list_current_page.set(usize::min(*tournament_list_total_pages, *tournament_list_current_page + 1)))
                            }
                        >{"Next"}</button>
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
