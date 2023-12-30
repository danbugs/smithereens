use yew::{function_component, html, Html, Properties};

use crate::models::{Player, Set, Tournament};

pub mod player_profile_header;
use crate::components::player_profile::player_profile_header::PlayerProfileHeader;

pub mod player_profile_tournament_list;
use crate::components::player_profile::player_profile_tournament_list::PlayerProfileTournamentList;

pub mod player_profile_summary_data;
use crate::components::player_profile::player_profile_summary_data::PlayerProfileSummaryData;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub display: bool,
    pub selected_player: Option<Player>,
    pub selected_player_summary_data: Option<(String, String, String, String, Vec<String>)>,
    pub selected_player_tournaments: Option<Vec<Tournament>>,
    pub selected_tournament_sets: Option<Vec<Set>>,
}

#[function_component(PlayerProfile)]
pub fn player_profile(props: &Props) -> Html {
    if props.display {
        let selected_player = props.selected_player.clone().unwrap();
        html! {
            <div>
            <br/>
            <div class="col-md-12 mb-5" style="background-color:#F3F4F6">
                <div class="container">
                    <div class="row">
                        <div class="col-md-12">
                            <PlayerProfileHeader
                                selected_player={selected_player.clone()}
                                display={props.selected_player_summary_data.is_some()}
                                selected_player_summary_data={props.selected_player_summary_data.clone()}
                            />
                            <hr/>
                            <PlayerProfileSummaryData
                                display={props.selected_player_summary_data.is_some()}
                                selected_player_summary_data={props.selected_player_summary_data.clone()}
                            />
                            <hr/>
                            <PlayerProfileTournamentList
                                display={props.selected_player_tournaments.is_some()}
                                selected_player_tournaments={props.selected_player_tournaments.clone()}
                                selected_tournament_sets={props.selected_tournament_sets.clone()}
                            />
                        </div>
                    </div>
                </div>
            </div>
            </div>
        }
    } else {
        html! {
            <div></div>
        }
    }
}
