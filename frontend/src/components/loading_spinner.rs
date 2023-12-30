use yew::{function_component, html, Html};

#[function_component(LoadingSpinner)]
pub fn loading_spinner() -> Html {
    html! {
        <div>
            <div class="d-flex justify-content-center">
                <div class="spinner-border" role="status">
                </div>
            </div>
            <br/>
            <br/>
        </div>
    }
}
