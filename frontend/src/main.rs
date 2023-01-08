use gloo_net::http::Request;
use models::{Player, Tournament};
use utils::parse_text_vector;
use wasm_bindgen::JsCast;
use web_sys::{HtmlButtonElement, HtmlInputElement};
use yew::prelude::*;

mod components;
use crate::components::{
    footer::Footer, header::Header, navbar::NavBar, player_list::PlayerList,
    player_profile::PlayerProfile,
};

mod models;
mod utils;

enum Msg {
    Search(String),
    DisplaySearch(Vec<Player>),
    SelectPlayer(i32),
    GetTournaments(Vec<Tournament>, i32),
    GetSummaryData(String, String, String, String, Vec<String>),
}

struct App {
    search_results: Vec<Player>,
    selected_player: Option<Player>,
    selected_player_tournaments: Option<Vec<Tournament>>,
    selected_player_summary_data: Option<(String, String, String, String, Vec<String>)>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            search_results: Vec::new(),
            selected_player: None,
            selected_player_tournaments: None,
            selected_player_summary_data: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Search(value) => {
                self.selected_player = None;
                self.selected_player_tournaments = None;
                self.selected_player_summary_data = None;
                self.search_results = Vec::new();

                let link = ctx.link().clone();

                wasm_bindgen_futures::spawn_local(async move {
                    let endpoint = format!("{}/players/{}", env!("SERVER_ADDRESS"), value);
                    let fetched_players: Vec<Player> = Request::get(&endpoint)
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();

                    link.send_message(Msg::DisplaySearch(fetched_players));
                });

                false
            }
            Msg::DisplaySearch(players) => {
                let mut ps = players.clone();
                ps.sort_by_key(|e| e.player_id);
                self.search_results = ps;

                true
            }
            Msg::SelectPlayer(pid) => {
                self.selected_player = Some(
                    self.search_results
                        .iter()
                        .find(|p| p.player_id == pid)
                        .unwrap()
                        .clone(),
                );

                let link = ctx.link().clone();

                wasm_bindgen_futures::spawn_local(async move {
                    let endpoint = format!("{}/tournaments/{}", env!("SERVER_ADDRESS"), pid);
                    let fetched_tournaments: Vec<Tournament> = Request::get(&endpoint)
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();

                    link.send_message(Msg::GetTournaments(fetched_tournaments, pid));
                });

                true
            }
            Msg::GetTournaments(tournaments, pid) => {
                let num_tournaments = tournaments.len();
                let mut ts = tournaments.clone();
                ts.sort_by_key(|e| e.tournament_id);
                ts.reverse();
                self.selected_player_tournaments = Some(ts);

                let link = ctx.link().clone();
                wasm_bindgen_futures::spawn_local(async move {
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

                    // console log two two chars
                    web_sys::console::log_1(&format!("Top two chars: {}", fetch_top_two_characters).into());

                    let wins_losses =
                        format!("{}-{}", fetch_wins_without_dqs, fetch_losses_without_dqs);
                    let tournaments_attended = format!("{}", num_tournaments);

                    link.send_message(Msg::GetSummaryData(
                        fetch_winrate,
                        fetch_competitor_type,
                        wins_losses,
                        tournaments_attended,
                        parse_text_vector(&fetch_top_two_characters)
                    ));
                });

                false
            }
            Msg::GetSummaryData(winrate, competitor_type, wins_losses, tournaments_attended, top_two_chars) => {
                self.selected_player_summary_data =
                    Some((winrate, competitor_type, wins_losses, tournaments_attended, top_two_chars));

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link().clone();
        let onkeypress = link.batch_callback(|e: KeyboardEvent| {
            if e.key() == "Enter" {
                let input: HtmlInputElement = e.target_unchecked_into();
                let value = input.value();
                input.set_value("");
                Some(Msg::Search(value))
            } else {
                None
            }
        });

        let onclick = link.batch_callback(|e: MouseEvent| {
            if let Some(elem) = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlButtonElement>().ok())
            {
                Some(Msg::SelectPlayer(elem.id().parse().unwrap()))
            } else {
                None
            }
        });

        html! {
            <div>
                <header>
                    <NavBar {onkeypress}/>
                    <Header display={self.selected_player.is_none()}/>
                </header>
                <div class="container">
                    <div class="row">
                        <PlayerList
                            full_display={self.selected_player.is_none() && !self.search_results.is_empty()}
                            alternate_display={self.selected_player.is_none()}
                            search_results={self.search_results.clone()}
                            {onclick}
                        />

                        <PlayerProfile
                            display={self.selected_player.is_some()}
                            selected_player={self.selected_player.clone()}
                            selected_player_tournaments={self.selected_player_tournaments.clone()}
                            selected_player_summary_data={self.selected_player_summary_data.clone()}
                        />
                    </div>
                </div>
                <footer>
                    <Footer />
                </footer>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
