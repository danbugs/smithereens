use web_sys::{KeyboardEvent, HtmlInputElement};
use yew::{function_component, html, Callback, Html, TargetCast};
use yew_router::{hooks::use_navigator, components::Link};

use crate::Route;

#[function_component(NavBar)]
pub fn navbar() -> Html {
    let navigator = use_navigator().unwrap();
    let onkeypress = Callback::from(move |e: KeyboardEvent| {
        if e.key() == "Enter" {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            input.set_value("");
            navigator.replace(&Route::PlayerList {
                gamer_tag: value.clone(),
            });
        }
    });

    html! {
        <nav class="navbar navbar-light" style="background-color:#C6263E">
            <div class="container-fluid">
                <div class="navbar-brand">
                    <Link<Route> to={Route::Home}>
                        <img src="/assets/smithe_logo-white_variant.png" style="width: 50%; height: 50%" alt="logo" class="d-inline-block align-text-top"/>
                    </Link<Route>>
                </div>
                <div class="navbar-nav ms-auto">
                    <div class="input-group">
                        <input type="text" class="form-control" placeholder="Search for a player" {onkeypress}/>
                    </div>
                </div>
            </div>
        </nav>
    }
}
