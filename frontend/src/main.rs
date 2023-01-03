use country_emoji::flag;
use gloo_net::http::Request;
use models::{Player, Tournament};
use web_sys::HtmlInputElement;
use yew::prelude::*;

mod models;

enum Msg {
    Search(String),
    DisplaySearch(Vec<Player>),
    SelectPlayer(Player),
    SetTournaments(Vec<Tournament>),
}

struct App {
    search_results: Vec<Player>,
    selected_player: Option<Player>,
    selectec_player_tournaments: Option<Vec<Tournament>>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            search_results: Vec::new(),
            selected_player: None,
            selectec_player_tournaments: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Search(value) => {
                self.selected_player = None;
                self.selectec_player_tournaments = None;

                let link = ctx.link().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let endpoint = format!("http://127.0.0.1:8000/players/{}", value);
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
            Msg::SelectPlayer(player) => {
                let value = player.player_id;
                self.selected_player = Some(player);

                let link = ctx.link().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let endpoint = format!("http://127.0.0.1:8000/tournaments/{}", value);
                    let fetched_tournaments: Vec<Tournament> = Request::get(&endpoint)
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();

                    link.send_message(Msg::SetTournaments(fetched_tournaments));
                });

                true
            }
            Msg::SetTournaments(tournaments) => {
                self.selectec_player_tournaments = Some(tournaments);
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
        html! {
        <div>
            <header>
                <nav class="navbar navbar-light" style="background-color:#C6263E">
                        <div class="container-fluid">
                            <a class="navbar-brand" href="">
                                <img src="assets/smithe_logo-white_variant.png" style="width: 50%; height: 50%" alt="logo" class="d-inline-block align-text-top"/>
                            </a>
                        <div class="navbar-nav ms-auto">
                            <div class="input-group">
                                <input type="text" class="form-control" placeholder="Search for a player" {onkeypress}/>
                            </div>
                        </div>
                    </div>
                </nav>
                {
                    if self.selected_player.is_none() {
                        html! {
                            <div class="text-center p-5" style="background-color:#C6263E;">
                                <img src="assets/smithe_header.svg" class="img-fluid" alt="" />
                            </div>
                        }
                    } else {
                        html! {
                            <div></div>
                        }
                    }
                }
            </header>
            <div class="container">
                <div class="row">
                    {
                        if self.selected_player.is_none() && !self.search_results.is_empty() {
                            html! {
                                <div class="col-md-12 mb-5">
                                    <br/>
                                    <br/>
                                    <ul class="list-group list-group-hover list-group-striped">
                                    {
                                        self.search_results.iter().map(|p| {
                                            html! {
                                                <button type="button"
                                                    class="list-group-item d-flex justify-content-between align-items-center"
                                                    onclick={
                                                        let player = p.clone();
                                                        link.callback_once(move |_| Msg::SelectPlayer(player))
                                                    }
                                                    key={p.player_id}>
                                                        <div class="d-flex align-items-center">
                                                        <img referrerpolicy="no-referrer" src={
                                                            if let Some(ppp) = p.profile_picture.clone() {
                                                                ppp
                                                            } else {
                                                                // placeholder round red profile picture from the internet
                                                                "https://i.imgur.com/78M17SL.png".to_string()
                                                            }} alt="profile_picture" style="width: 45px; height: 45px"
                                                            class="rounded-circle" />
                                                        <div class="ms-3">
                                                            <p class="fw-bold mb-1">
                                                            {
                                                                if p.prefix.is_none() || p.prefix.as_ref().unwrap().is_empty() {
                                                                    format!("{}", &p.gamer_tag)
                                                                } else {
                                                                    format!("{} | {}", p.prefix.as_ref().unwrap(), &p.gamer_tag)
                                                                }
                                                            }</p>
                                                        </div>
                                                    </div>
                                                    <span>
                                                        {
                                                            if let Some(pc) = p.country.clone() {
                                                                flag(&pc).unwrap_or("".to_string())
                                                            } else {
                                                                "".to_string()
                                                            }
                                                        }
                                                    </span>
                                                </button>
                                            }
                                        }).collect::<Html>()
                                    }
                                    </ul>
                                    <br/>
                                </div>
                            }
                        } else if self.selected_player.is_none() {
                            html! {
                                <div class="text-center" style="color:#C6263E">
                                    <br/>
                                    <br/>
                                    <h2>{"• • •"}</h2>
                                </div>
                            }
                        } else {
                            html! {
                                <div></div>
                            }
                        }
                    }
                    {
                        if self.selected_player.is_some() {
                            let sp = self.selected_player.as_ref().unwrap();
                            html! {
                                <div>
                                    <br/>
                                    <div class="col-md-12 mb-5" style="background-color:#F3F4F6">
                                        // display image and gamer tag side by side
                                        <div class="container">
                                            <br/>
                                            <br/>
                                            <div class="row">
                                                // align profile picture in center of row
                                                <div class="col-auto align-self-center">
                                                    <img src={sp.profile_picture.as_ref().unwrap().clone()} alt="Profile Picture" class="img-fluid rounded-circle" style="max-width:128px"/>
                                                </div>
                                                <div class="col-auto">
                                                    <h2>{
                                                        if sp.prefix.is_none() || sp.prefix.as_ref().unwrap().is_empty() {
                                                            format!("{}", &sp.gamer_tag)
                                                        } else {
                                                            format!("{} | {}", sp.prefix.as_ref().unwrap(), &sp.gamer_tag)
                                                        }
                                                    }</h2>
                                                    // display name if it exists, otherwise nothing
                                                    {
                                                        if sp.name.is_some() && !sp.name.as_ref().unwrap().is_empty() {
                                                            html! {
                                                                // display name and flag in a p in grey
                                                                <p class="text-muted font-weight-light">{
                                                                    format!("{} {}", 
                                                                        sp.name.as_ref().unwrap(), 
                                                                        if let Some(pc) = sp.country.clone() {
                                                                            flag(&pc).unwrap_or("".to_string())
                                                                        } else {
                                                                            "".to_string()
                                                                        })}
                                                                </p>
                                                            }
                                                        } else {
                                                            html! {
                                                                <div></div>
                                                            }
                                                        }
                                                    }

                                                    // display country in a p element if it is some, otherwise nothing
                                                    {if sp.gender_pronouns.is_some() {
                                                        html! {
                                                            <p class="text-muted font-weight-light">{&sp.gender_pronouns.as_ref().unwrap()}</p>
                                                        }
                                                    } else {
                                                        html! {
                                                            <div></div>
                                                        }
                                                    }}

                                                    // display link to twitter if it is some, otherwise nothing
                                                    {if sp.twitter_username.is_some() {
                                                        html! {
                                                        <a target="_blank" href={format!("https://twitter.com/{}", sp.twitter_username.as_ref().unwrap_or(&String::from("")))}>
                                                            // img twitter dark icon 32px
                                                            <img src="https://abs.twimg.com/favicons/twitter.ico" alt="Twitter" class="img-fluid" style="max-width:32px"/>
                                                        </a>
                                                        }
                                                    } else {
                                                        html! {
                                                            <div></div>
                                                        }
                                                    }}

                                                    // display link to twitter if it is some, otherwise nothing
                                                    {if sp.twitch_username.is_some() {
                                                        html! {
                                                        <a target="_blank" href={format!("https://twitch.tv/{}", sp.twitch_username.as_ref().unwrap_or(&String::from("")))}>
                                                            <img src="https://static.twitchcdn.net/assets/favicon-32-d6025c14e900565d6177.png" alt="Twitch" class="img-fluid" style="max-width:32px"/>
                                                        </a>
                                                        }
                                                    } else {
                                                        html! {
                                                            <div></div>
                                                        }
                                                    }}
                                                </div>                                                
                                            </div>

                                            <br/>
                                            <hr/>
                                            <br/>

                                            {
                                                if self.selectec_player_tournaments.is_none() {
                                                    html! {
                                                        // centered bootstrap spinner
                                                        <div>
                                                            <div class="d-flex justify-content-center">
                                                                <div class="spinner-border" role="status">
                                                                </div>
                                                            </div>
                                                            <br/>
                                                            <br/>
                                                        </div>
                                                    }
                                                } else {
                                                    html! {
                                                        <div>
                                                            <ul class="list-group list-group-light">
                                                            {
                                                                self.selectec_player_tournaments.as_ref().unwrap().iter().map(|t| {
                                                                    html! {
                                                                        <li class="list-group-item d-flex justify-content-between align-items-center">
                                                                            // display event name, seed, placement, and number of entrants
                                                                            <div>
                                                                                <div class="fw-bold">{format!("{} | {} entrants", &t.event_name, &t.num_entrants)}</div>
                                                                                <hr/>
                                                                                <div class="text-muted">{format!("Seed: {} | Placement: {}", t.seed, t.placement)}</div>
                                                                            </div>
                                                                        </li>
                                                                    }
                                                                }).collect::<Html>()
                                                            }
                                                            </ul>
                                                            <br/>
                                                            <br/>
                                                        </div>
                                                    }
                                                }
                                            }
                                        </div>
                                    </div>
                                    <br/>
                                    <br/>
                                </div>
                            }
                        } else {
                            html! {
                                <div>
                                </div>
                            }
                        }
                    }
                </div>
            </div>
            // bootstrap footer
            // make footer sticked to bottom of page
            <footer class="footer fixed-bottom" style="background-color:#D2D2D2;">
                <div class="container" style="color:#C6263E;">
                    <center>
                        <small style="font-size: 0.8rem;">
                            {"Made with "}
                            // icon for made with love
                            <img src="https://img.icons8.com/ios/50/000000/like--v1.png" alt="Made with love" class="img-fluid" style="max-width:16px"/>
                            {", "}
                            // icon for rust
                            <img src="https://devicons.railway.app/i/rust-dark.svg" alt="Rust" class="img-fluid" style="max-width:16px"/>
                            {", and "}
                            // icon for webassembly
                            <img src="https://devicons.railway.app/i/wasm.svg" alt="WebAssembly" class="img-fluid" style="max-width:16px"/>
                            // by
                            {" by "}
                            // link to my twitter
                            <a class="link-danger" target="_blank" href="https://twitter.com/dantotto">
                            {"@dantotto"}
                            </a>
                            {"."}
                        </small>
                        // in a new line mention that the code is open-source and link its' repository
                        <br/>
                        <small style="font-size: 0.8rem;">
                            {"This website's code is open-source and can be found "}
                            <a class="link-danger" target="_blank" href="https://github.com/danbugs/smithereens">
                                {"here"}
                            </a>
                            {"."}
                        </small>
                    </center>
                </div>
            </footer>
        </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
