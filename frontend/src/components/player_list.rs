use country_emoji::flag;
use gloo_net::http::Request;

use yew::{function_component, html, use_effect_with, use_state, Callback, Html, Properties};
use yew_router::hooks::use_navigator;

use crate::{
    components::loading_spinner::LoadingSpinner, models::Player, utils::create_page_numbers,
};

const PAGE_SIZE: usize = 10; // Number of items per page

#[derive(Properties, PartialEq)]
pub struct Props {
    pub gamer_tag: String,
}

#[function_component(PlayerList)]
pub fn player_list(props: &Props) -> Html {
    let navigator = use_navigator().unwrap();

    let gamer_tag = props.gamer_tag.clone();
    let loading = use_state(|| false);
    let search_results = use_state(Vec::<Player>::new);

    let current_page = use_state(|| 1);
    let total_pages = use_state(|| 0);
    let start_index = use_state(|| 0);
    let end_index = use_state(|| 0);

    let paginated_results = use_state(Vec::<Player>::new);

    {
        let search_results = search_results.clone();
        let loading = loading.clone();
        let total_pages = total_pages.clone();
        let end_index = end_index.clone();
        let paginated_results = paginated_results.clone();

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

                total_pages.set((fetched_players.len() as f32 / PAGE_SIZE as f32).ceil() as usize);
                search_results.set(fetched_players.clone());

                let end = usize::min(PAGE_SIZE, fetched_players.len());

                end_index.set(end);

                paginated_results.set(fetched_players[0..end].to_vec());

                loading.set(false);
            });
        });
    }

    {
        let start_index = start_index.clone();
        let end_index = end_index.clone();
        let paginated_results = paginated_results.clone();
        let current_page = current_page.clone();
        let search_results = search_results.clone();

        use_effect_with(current_page.clone(), move |_| {
            let start = (*current_page - 1) * PAGE_SIZE;
            let end = usize::min(start + PAGE_SIZE, (*search_results).len());

            start_index.set(start);
            end_index.set(end);

            let search_results = search_results.clone();
            let paginated_results = paginated_results.clone();

            paginated_results.set((*search_results)[start..end].to_vec());
        });
    }

    let curr_page = *current_page;
    let tot_pages = *total_pages;

    let pagination_numbers = create_page_numbers(curr_page, tot_pages);

    html! {
        <>
        if *loading {
            <div class="col-md-12 mb-5">
                <br/>
                <br/>
                <LoadingSpinner/>
            </div>
        } else {
            if (*paginated_results).is_empty() {
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
                        (*paginated_results).iter().map(|p| {
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
                                                    p.gamer_tag.to_string()
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
                <nav>
                    <ul class="pagination justify-content-center">
                        <li class={if *current_page == 1 { "page-item disabled" } else { "page-item" }}>
                        <button class="page-link" onclick={
                                let current_page = current_page.clone();
                                Callback::from(move |_| current_page.set(usize::max(1, *current_page - 1)))
                            }>{"Previous"}</button>
                        </li>
                        {
                            for pagination_numbers.iter().map(|&num| {
                                if num == 0 {
                                    html! { <li class="page-item disabled"><span class="page-link">{"..."}</span></li> }
                                } else {
                                    let is_active = num == *current_page;
                                    html! {
                                        <li class={if is_active { "page-item active" } else { "page-item" }}>
                                        <button class="page-link" onclick={
                                                    let current_page = current_page.clone();
                                                    Callback::from(move |_| current_page.set(num))}>
                                                { num.to_string() }
                                            </button>
                                        </li>
                                    }
                                }
                            })
                        }
                        <li class={if *current_page == *total_pages { "page-item disabled" } else { "page-item" }}>
                        <button class="page-link" onclick={
                                let current_page = current_page.clone();
                                Callback::from(move |_| current_page.set(usize::min(*total_pages, *current_page + 1)))
                            }>{"Next"}</button>
                        </li>
                    </ul>
                </nav>

                <br/>
                </div>
            }
        }
        </>
    }
}
