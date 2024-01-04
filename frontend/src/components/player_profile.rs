use gloo_net::http::Request;
use yew::{function_component, html, use_effect_with, use_state, Html, Properties};

use crate::components::player_profile::player_profile_head_to_head_list::PlayerProfileHeadToHeadList;
use crate::models::{HeadToHeadResult, Player, Set, Tournament};

pub mod player_profile_header;
use crate::components::player_profile::player_profile_header::PlayerProfileHeader;

pub mod player_profile_tournament_list;
use crate::components::player_profile::player_profile_tournament_list::PlayerProfileTournamentList;

pub mod player_profile_summary_data;
use crate::components::player_profile::player_profile_summary_data::PlayerProfileSummaryData;
use crate::utils::parse_text_vector;

pub mod player_profile_head_to_head_list;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub player_id: i32,
}

#[function_component(PlayerProfile)]
pub fn player_profile(props: &Props) -> Html {
    let pid = props.player_id;
    let selected_player = use_state(|| None::<Player>);
    let selected_player_summary_data =
        use_state(|| None::<(String, String, String, String, Vec<String>)>);
    let selected_player_tournaments = use_state(|| None::<Vec<Tournament>>);
    let selected_tournament_sets = use_state(|| None::<Vec<Set>>);
    let selected_player_head_to_heads = use_state(|| None::<Vec<HeadToHeadResult>>);

    {
        let selected_player = selected_player.clone();
        let selected_player_summary_data = selected_player_summary_data.clone();
        let selected_player_tournaments = selected_player_tournaments.clone();
        let selected_tournament_sets = selected_tournament_sets.clone();
        let selected_player_head_to_heads = selected_player_head_to_heads.clone();

        use_effect_with((), move |_| {
            let selected_player = selected_player.clone();
            let selected_player_summary_data = selected_player_summary_data.clone();
            let selected_player_tournaments = selected_player_tournaments.clone();
            let selected_tournament_sets = selected_tournament_sets.clone();

            wasm_bindgen_futures::spawn_local(async move {
                // get player
                let fetched_player: Player =
                    Request::get(&format!("{}/player/{}", env!("SERVER_ADDRESS"), pid))
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();

                selected_player.set(Some(fetched_player));

                // get tournaments, and sets
                let mut fetched_tournaments: Vec<Tournament> =
                    Request::get(&format!("{}/tournaments/{}", env!("SERVER_ADDRESS"), pid))
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();

                fetched_tournaments.sort_by_key(|e| e.tournament_id);
                fetched_tournaments.reverse();

                let fetched_sets: Vec<Set> =
                    Request::get(&format!("{}/sets/{}", env!("SERVER_ADDRESS"), pid))
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();

                let num_tournaments = fetched_tournaments.len();

                selected_player_tournaments.set(Some(fetched_tournaments));
                selected_tournament_sets.set(Some(fetched_sets));

                // get head to head data
                let mut fetch_head_to_heads: Vec<HeadToHeadResult> = Request::get(&format!(
                    "{}/player/{}/head_to_head",
                    env!("SERVER_ADDRESS"),
                    pid
                ))
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();

                // sort by total sets
                fetch_head_to_heads.sort_by_key(|e| e.total_sets);
                fetch_head_to_heads.reverse();

                selected_player_head_to_heads.set(Some(fetch_head_to_heads));

                // get summary data
                let fetch_winrate: String =
                    Request::get(&format!("{}/sets/{}/winrate", env!("SERVER_ADDRESS"), pid))
                        .send()
                        .await
                        .unwrap()
                        .text()
                        .await
                        .unwrap();

                let fetch_competitor_type: String = Request::get(&format!(
                    "{}/sets/{}/competitor_type",
                    env!("SERVER_ADDRESS"),
                    pid
                ))
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();

                let fetch_wins_without_dqs: String = Request::get(&format!(
                    "{}/sets/{}/wins_without_dqs",
                    env!("SERVER_ADDRESS"),
                    pid
                ))
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();

                let fetch_losses_without_dqs: String = Request::get(&format!(
                    "{}/sets/{}/losses_without_dqs",
                    env!("SERVER_ADDRESS"),
                    pid
                ))
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();

                let fetch_top_two_characters: String = Request::get(&format!(
                    "{}/player/{}/top_two_characters",
                    env!("SERVER_ADDRESS"),
                    pid
                ))
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();

                let wins_losses =
                    format!("{}-{}", fetch_wins_without_dqs, fetch_losses_without_dqs);
                let tournaments_attended = format!("{}", num_tournaments);

                selected_player_summary_data.set(Some((
                    fetch_winrate,
                    fetch_competitor_type,
                    wins_losses,
                    tournaments_attended,
                    parse_text_vector(&fetch_top_two_characters),
                )));
            });
        });
    }

    html! {
        <div>
            <br/>
            <div class="col-md-12 mb-5" style="background-color:#F3F4F6">
                <div class="container">
                    <div class="row">
                        <div class="col-md-12">
                            <PlayerProfileHeader
                                selected_player={(*selected_player).clone()}
                                display={(*selected_player_summary_data).is_some()}
                                selected_player_summary_data={(*selected_player_summary_data).clone()}
                            />
                            <hr/>
                            <PlayerProfileSummaryData
                                display={(*selected_player_summary_data).is_some()}
                                selected_player_summary_data={(*selected_player_summary_data).clone()}
                            />
                            <hr/>
                            <PlayerProfileTournamentList
                                display={(*selected_player_tournaments).is_some()}
                                selected_player_tournaments={(*selected_player_tournaments).clone()}
                                selected_tournament_sets={(*selected_tournament_sets).clone()}
                            />

                            <PlayerProfileHeadToHeadList
                                display={(*selected_player_head_to_heads).is_some()}
                                selected_player_head_to_heads={(*selected_player_head_to_heads).clone()}
                            />
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
