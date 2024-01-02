use country_emoji::flag;
use gloo_net::http::Request;

use yew::{function_component, html, use_effect_with, use_state, Callback, Html, Properties};
use yew_router::hooks::use_navigator;

use crate::{components::loading_spinner::LoadingSpinner, models::Player};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub gamer_tag: String,
}

#[function_component(PlayerList)]
pub fn player_list(props: &Props) -> Html {
    let navigator = use_navigator().unwrap();

    let gamer_tag = props.gamer_tag.clone();
    let loading = use_state(|| false);
    let search_results = use_state(|| Vec::<Player>::new());

    {
        let search_results = search_results.clone();
        let loading = loading.clone();

        use_effect_with(gamer_tag.clone(), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);

                let endpoint = format!("{}/players/{}", env!("SERVER_ADDRESS"), gamer_tag);
                let fetched_players: Vec<Player> = Request::get(&endpoint)
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

                web_sys::console::log_1(&format!("fetched_players: {:#?}", fetched_players).into());
                search_results.set(fetched_players);

                loading.set(false);
            });
        });
    }

    html! {
        <>
        if *loading {
            <div class="col-md-12 mb-5">
                <br/>
                <br/>
                <LoadingSpinner/>
            </div>
        } else {
            if (*search_results).is_empty() {
                <div class="text-center" style="color:#C6263E">
                    <br/>
                    <br/>
                    <h2>{"No results"}</h2>
                </div>
            } else {
                <div class="col-md-12 mb-5">
                <br/>
                <br/>
                    <ul class="list-group list-group-hover list-group-striped">
                    {
                        (*search_results).iter().map(|p| {
                            let player_id = p.player_id;
                            let navigator = navigator.clone();
                            let onclick = Callback::from(move |_| {
                                navigator.replace(&crate::Route::PlayerProfile { player_id });
                            });
                            html! {
                                <button type="button"
                                    class="list-group-item d-flex justify-content-between align-items-center"
                                    {onclick}
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
                                                    (&p.gamer_tag).to_string()
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
        }
        </>
    }
}
