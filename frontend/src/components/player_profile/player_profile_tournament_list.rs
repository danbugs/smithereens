use yew::{function_component, html, Properties, Html};

use crate::models::Tournament;

use crate::components::loading_spinner::LoadingSpinner;
use crate::utils::calculate_spr;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub display: bool,
    pub selected_player_tournaments: Option<Vec<Tournament>>,
}

#[function_component(PlayerProfileTournamentList)]
pub fn player_profile_tournament_list(props: &Props) -> Html {
    if props.display {
        html! {
            html! {
                <div>
                    <ul class="list-group list-group-light">
                    {
                        props.selected_player_tournaments.as_ref().unwrap().iter().map(|t| {
                            html! {
                                <div>
                                    <li class="list-group-item d-flex justify-content-between align-items-center">
                                        // display event name, seed, placement, and number of entrants
                                        <div class="col-md-12">
                                            <h4 class="fw-bold">{format!("{}", &t.event_name)}</h4>
                                            <hr/>
                                            <div class="row">
                                                <div class="col-md-2">
                                                    <div class="card border-0 text-center">
                                                        <div class="card-body">
                                                            <h5 class="card-title fw-bold">{"Seed"}</h5>
                                                            <p class="card-text">{&t.seed}</p>
                                                        </div>
                                                    </div>
                                                </div>
                                
                                                <div class="col-md-2">
                                                    <div class="card border-0 text-center">
                                                        <div class="card-body">
                                                            <h5 class="card-title fw-bold">{"Placement"}</h5>
                                                            <p class="card-text">{format!("{}/{}", &t.placement, &t.num_entrants)}</p>
                                                        </div>
                                                    </div>
                                                </div>

                                                <div class="col-md-2">
                                                    <div class="card border-0 text-center">
                                                        <div class="card-body">
                                                            <h5 class="card-title fw-bold">{"SPR"}</h5>
                                                            <p class="card-text">{format!("{}", calculate_spr(t.seed, t.placement))}</p>
                                                        </div>
                                                    </div>
                                                </div>                                                
                                            </div>
                                            <br/>
                                        </div>
                                    </li>
                                    <br/>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                    </ul>
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