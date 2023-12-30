use yew::{function_component, html, Html, Properties};

use crate::models::{Set, Tournament};

use crate::components::loading_spinner::LoadingSpinner;
use crate::utils::calculate_spr_or_uf;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub display: bool,
    pub selected_player_tournaments: Option<Vec<Tournament>>,
    pub selected_tournament_sets: Option<Vec<Set>>,
}

#[function_component(PlayerProfileTournamentList)]
pub fn player_profile_tournament_list(props: &Props) -> Html {
    if props.display {
        html! {
            html! {
                <div>
                    <div class="accordion" id="accordion">
                    {
                        props.selected_player_tournaments.as_ref().unwrap().iter().map(|t| {
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
