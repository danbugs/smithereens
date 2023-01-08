use yew::{function_component, html, Properties};

use crate::components::loading_spinner::LoadingSpinner;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub display: bool,
    pub selected_player_summary_data: Option<(String, String, String, String, Vec<String>)>,
}

#[function_component(PlayerProfileSummaryData)]
pub fn player_profile_summary_data(props: &Props) -> Html {
    if props.display {
        html! {
            <div class="row">
                <div class="col-md-3">
                    <div class="card text-center">
                        <div class="card-body">
                            <h5 class="card-title fw-bold">{"WIN RATE"}</h5>
                            <p class="card-text">{format!("{}%", props.selected_player_summary_data.clone().unwrap().0)}</p>
                        </div>
                    </div>
                </div>

                <div class="col-md-3">
                    <div class="card text-center">
                        <div class="card-body">
                            <h5 class="card-title fw-bold">{"WIN-LOSSES"}</h5>
                            <p class="card-text">{props.selected_player_summary_data.clone().unwrap().2}</p>
                        </div>
                    </div>
                </div>

                <div class="col-md-3">
                    <div class="card text-center">
                        <div class="card-body">
                            <h5 class="card-title fw-bold">{"COMPETITOR TYPE"}</h5>
                            <p class="card-text">{props.selected_player_summary_data.clone().unwrap().1}</p>
                        </div>
                    </div>
                </div>

                <div class="col-md-3">
                    <div class="card text-center">
                        <div class="card-body">
                            <h5 class="card-title fw-bold">{"TOURNAMENTS ENTERED"}</h5>
                            <p class="card-text">{props.selected_player_summary_data.clone().unwrap().3}</p>
                        </div>
                    </div>
                </div>
            </div>
        }
    } else {
        html! {
            <LoadingSpinner/>
        }
    }
}
