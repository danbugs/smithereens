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
    web_sys::console::log_1(&format!("route: {:#?}", routes).into());
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
                    <div class="text-center" style="color:#C6263E">
                        <br/>
                        <br/>
                        <h2>{"• • •"}</h2>
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
