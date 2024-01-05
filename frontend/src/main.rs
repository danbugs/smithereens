use yew::prelude::*;
use yew_router::prelude::*;

mod components;
use crate::components::{
    footer::Footer, header::Header, navbar::NavBar, player_list::PlayerList,
    player_profile::PlayerProfile,
};

mod models;
mod utils;

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/search/:gamer_tag")]
    PlayerList { gamer_tag: String },
    #[at("/player/:player_id")]
    PlayerProfile { player_id: i32 },
    #[at("/404")]
    NotFound,
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch}/>
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    html! {
        <div>
        <header>
            <NavBar/>
            <Header/>
        </header>
        <div class="container">
            <div class="row">
            {
                match routes {
                    Route::Home => html! {
                        <div class="why-join-section my-5">
                            <div class="row align-items-start">
                                <div class="col-md-6">
                                    <h3 class="mb-3">{"Why Smithereens?"}</h3>
                                    <p class="lead">{"Accessible anywhere, Smithereens is the all-in-one tool for Smash analytics — whether you're a fan or a fierce competitor."}</p>
                                    <ul class="list-unstyled mb-4">
                                        <li><i class="bi bi-check2"></i>{" CLI & Web App"}</li>
                                        <li><i class="bi bi-check2"></i>{" Fully Open-Source"}</li>
                                        <li><i class="bi bi-check2"></i>{" Independent & Community-Driven"}</li>
                                        <li><i class="bi bi-check2"></i>{" Continuously Tested & Reliable"}</li>
                                    </ul>
                                    <p class="mt-4 mb-2">{"Experience unique features:"}</p>
                                    <ul class="list-unstyled">
                                        <li><i class="bi bi-twitter"></i>{" Twitter Integration"}</li>
                                        <li><i class="bi bi-graph-up"></i>{" Insightful Metrics"}</li>
                                        <li><i class="bi bi-plus-circle"></i>{" And much more..."}</li>
                                    </ul>
                                </div>
                                <div class="col-md-6">
                                    <div class="discord-invite text-center py-5 px-4" style="background-color: #F8F9FA;">
                                        <h3 class="mb-3">{"Join Our Discord"}</h3>
                                        <p class="mb-4">{"Got suggestions or found a bug? We're all ears on Discord. Join our community and help shape Smithereens!"}</p>
                                        <a href="https://discord.gg/X935GRyyWE" class="btn btn-primary btn-lg" target="_blank">{"Join Discord"}</a>
                                    </div>
                                </div>
                            </div>
                        </div>
                    },
                    Route::PlayerList { gamer_tag } => html! {
                        <PlayerList
                            {gamer_tag}
                        />
                    },
                    Route::PlayerProfile { player_id } => html! {
                    <PlayerProfile
                        {player_id}
                    />
                    },
                    Route::NotFound => html! {
                        <div class="text-center" style="color:#C6263E">
                            <br/>
                            <br/>
                            <h2>{"Not Found"}</h2>
                        </div>
                    },
                }
            }
            </div>
        </div>
        <footer>
            <Footer />
        </footer>
    </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
