use web_sys::KeyboardEvent;
use yew::{function_component, html, Callback, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onkeypress: Callback<KeyboardEvent>,
}

#[function_component(NavBar)]
pub fn navbar(props: &Props) -> Html {
    html! {
        <nav class="navbar navbar-light" style="background-color:#C6263E">
            <div class="container-fluid">
                    <a class="navbar-brand" href="">
                        <img src="assets/smithe_logo-white_variant.png" style="width: 50%; height: 50%" alt="logo" class="d-inline-block align-text-top"/>
                    </a>
                <div class="navbar-nav ms-auto">
                    <div class="input-group">
                        <input type="text" class="form-control" placeholder="Search for a player" onkeypress={props.onkeypress.clone()}/>
                    </div>
                </div>
            </div>
        </nav>
    }
}
