use gloo_net::http::Request;
use models::Player;
use web_sys::HtmlInputElement;
use yew::prelude::*;

mod models;

enum Msg {
    Search(String),
    DisplaySearch(Vec<Player>),
}

struct App {
    search_results: Vec<Player>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            search_results: Vec::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Search(value) => {
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
                self.search_results = players;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
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
            <>
            <div class="bg-light">
                <div class="container">
                    <div class="col-md-12">
                        <h1>{"Smithereens"}</h1>
                        <input type="text" class="form-control input-lg" placeholder="Who do you want to search for?" {onkeypress}/>
                        <br/>
                    </div>
                </div>
            </div>
            <div class="container">
            {
                self.search_results.iter().map(|p| {
                    html!{
                        <>
                        <a href={format!("https://www.start.gg/{}", p.user_slug)} target="_blank" key={p.player_id}>{
                            if p.prefix.is_none() || p.prefix.as_ref().unwrap().is_empty() {
                                format!("{}", &p.gamer_tag)
                            } else {
                                format!("{} | {}", p.prefix.as_ref().unwrap(), &p.gamer_tag)
                            }
                        }</a><br/>
                        </>
                    }
                }).collect::<Html>()
            }
            </div>
            </>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
