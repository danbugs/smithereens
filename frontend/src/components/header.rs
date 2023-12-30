use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub display: bool,
}

#[function_component(Header)]
pub fn header(props: &Props) -> Html {
    if props.display {
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
