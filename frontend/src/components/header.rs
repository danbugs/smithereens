use yew::{function_component, html, Html};
use yew_router::hooks::use_location;

#[function_component(Header)]
pub fn header() -> Html {
    let location = use_location().unwrap();
    let display = if location.path() == "/" {
        true
    } else {
        false
    };

    if display {
        html! {
            <div class="text-center p-5" style="background-color:#C6263E;">
                <img src="assets/smithe_header.svg" width="75%" height="75%" class="img-fluid" alt="" />
            </div>
        }
    } else {
        html! {
            <div></div>
        }
    }
}
