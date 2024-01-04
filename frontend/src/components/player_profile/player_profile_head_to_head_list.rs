use yew::{function_component, Properties, Html, html, use_effect_with, use_state, Callback};

use crate::{models::HeadToHeadResult, utils::create_page_numbers, components::loading_spinner::LoadingSpinner};

const PAGE_SIZE: usize = 10; // Number of items per page

#[derive(Properties, PartialEq)]
pub struct Props {
    pub display: bool,
    pub selected_player_head_to_heads: Option<Vec<HeadToHeadResult>>,
}

#[function_component(PlayerProfileHeadToHeadList)]
pub fn player_profile_head_to_head_list(props: &Props) -> Html {
    let current_page = use_state(|| 1);
    let total_pages = use_state(|| 0);
    let start_index = use_state(|| 0);
    let end_index = use_state(|| 0);

    let paginated_head_to_heads = use_state(|| Vec::<HeadToHeadResult>::new());

    {
        let selected_player_head_to_heads = props.selected_player_head_to_heads.clone();
        let total_pages = total_pages.clone();
        let end_index = end_index.clone();
        let paginated_head_to_heads = paginated_head_to_heads.clone();

        use_effect_with(props.selected_player_head_to_heads.clone(), move |_| {
            // Calculate total pages
            let t_pages = selected_player_head_to_heads
                .as_ref()
                .map_or(0, |hxhs| {
                    (hxhs.len() as f32 / PAGE_SIZE as f32).ceil() as usize
                });

            // Update total pages state
            total_pages.set(t_pages);

            // Calculate paginated tournaments
            if let Some(hxhs) = selected_player_head_to_heads.as_ref() {
                let end = usize::min(PAGE_SIZE, hxhs.len());
                end_index.set(end);
                paginated_head_to_heads.set(hxhs[0..end].to_vec());
            }
        });
    }

    {
        let start_index = start_index.clone();
        let end_index = end_index.clone();
        let paginated_head_to_heads = paginated_head_to_heads.clone();
        let current_page = current_page.clone();
        let selected_player_head_to_heads = props.selected_player_head_to_heads.clone();
        use_effect_with(current_page.clone(), move |_| {
            if let Some(sphxhs) = selected_player_head_to_heads {
                let start = (*current_page - 1) * PAGE_SIZE;
                let end = usize::min(start + PAGE_SIZE, sphxhs.len());

                start_index.set(start);
                end_index.set(end);

                paginated_head_to_heads.set(sphxhs[start..end].to_vec());
            }
        });
    }

    let curr_page = *current_page;
    let tot_pages = *total_pages;
    let pagination_numbers = create_page_numbers(curr_page, tot_pages);

    html! {
        if !props.display {
            <LoadingSpinner/>
        } else if (*paginated_head_to_heads).is_empty() {
            <div class="text-center" style="color:#C6263E">
                <br/>
                <br/>
                <h2>{"No head-to-head data available"}</h2>
            </div>
        } else {
            <>
                <div class="col-md-12 mb-5">
                    <div class="head-to-head-title">
                        <h3 class="text-center text-uppercase font-weight-bold">{"Head-to-heads"}</h3>
                        <hr class="my-4"/> // Stylish horizontal rule to separate title from content
                    </div>
                    <ul class="list-group list-group-hover list-group-striped">
                    {
                        for (*paginated_head_to_heads).iter().map(|h2h| {
                            html! {
                                <li class="list-group-item d-flex justify-content-between align-items-center">
                                    <span>{ &h2h.opponent_tag }</span>
                                    <div>
                                        <span class="badge bg-primary rounded-pill">{ format!("Wins: {}", &h2h.wins) }</span>
                                        <span class="badge bg-danger rounded-pill">{ format!("Losses: {}", &h2h.losses) }</span>
                                    </div>
                                </li>
                            }
                        })
                    }
                    </ul>
                <br/>
                </div>
                if *total_pages > 1 {
                    <nav>
                        <ul class="pagination justify-content-center">
                            <li class={if curr_page == 1 { "page-item disabled" } else { "page-item" }}>
                                <button class="page-link" onclick={
                                    let current_page = current_page.clone();
                                    Callback::from(move |_| current_page.set(usize::max(1, curr_page - 1)))
                                }>{"Previous"}</button>
                            </li>
                            {
                                for pagination_numbers.iter().map(|&num| {
                                    if num == 0 {
                                        html! { <li class="page-item disabled"><span class="page-link">{"..."}</span></li> }
                                    } else {
                                        let is_active = num == curr_page;
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
                            <li class={if curr_page == tot_pages { "page-item disabled" } else { "page-item" }}>
                                <button class="page-link" onclick={
                                    let current_page = current_page.clone();
                                    Callback::from(move |_| current_page.set(usize::min(tot_pages, curr_page + 1)))
                                }>{"Next"}</button>
                            </li>
                        </ul>
                    </nav>
                }
            </>
        }
    }
}