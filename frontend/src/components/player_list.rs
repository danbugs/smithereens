use country_emoji::flag;
use web_sys::MouseEvent;
use yew::{function_component, html, Properties, Html, Callback};

use crate::models::Player;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub full_display: bool,
    pub alternate_display: bool,
    pub search_results: Vec<Player>,
    pub onclick: Callback<MouseEvent>,
}

#[function_component(PlayerList)]
pub fn player_list(props: &Props) -> Html {
    if props.full_display {
        html! {
            <div class="col-md-12 mb-5">
            <br/>
            <br/>
            <ul class="list-group list-group-hover list-group-striped">
            {
                props.search_results.iter().map(|p| {
                    html! {
                        <button type="button"
                            class="list-group-item d-flex justify-content-between align-items-center"
                            onclick={props.onclick.clone()}
                            id={p.player_id.to_string()}
                            key={p.player_id}>
                                <div class="d-flex align-items-center">
                                <img referrerpolicy="no-referrer" src={
                                    if let Some(ppp) = p.profile_picture.clone() {
                                        ppp
                                    } else {
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
    } else if props.alternate_display {
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
