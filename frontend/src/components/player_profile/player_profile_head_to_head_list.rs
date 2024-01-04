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
    let head_to_heads = props.selected_player_head_to_heads.clone().unwrap_or_default();

    let current_page = use_state(|| 1);
    let total_pages = use_state(|| (head_to_heads.len() as f32 / PAGE_SIZE as f32).ceil() as usize);
    let paginated_head_to_heads = use_state(|| Vec::<HeadToHeadResult>::new());

    {
        let current_page = current_page.clone();
        let head_to_heads = head_to_heads.clone();
        let paginated_head_to_heads = paginated_head_to_heads.clone();
        use_effect_with(current_page.clone(), move |_| {
            let start = (*current_page - 1) * PAGE_SIZE;
            let end = usize::min(start + PAGE_SIZE, head_to_heads.len());
            paginated_head_to_heads.set(head_to_heads[start..end].to_vec());
            || ()
        });
    }

    let curr_page = *current_page;
    let tot_pages = *total_pages;
    let pagination_numbers = create_page_numbers(curr_page, tot_pages);

    html! {
        if !props.display {
            <LoadingSpinner/>
        } else if head_to_heads.is_empty() {
            <div class="text-center" style="color:#C6263E">
                <br/>
                <br/>
                <h2>{"No head-to-head data available"}</h2>
            </div>
        } else {
            <>
                <div class="col-md-12 mb-5">
                <br/>
                <br/>
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
                                <a class="page-link" href="#" onclick={
                                    let current_page = current_page.clone();
                                    Callback::from(move |_| current_page.set(usize::max(1, curr_page - 1)))
                                }>{"Previous"}</a>
                            </li>
                            {
                                for pagination_numbers.iter().map(|&num| {
                                    if num == 0 {
                                        html! { <li class="page-item disabled"><span class="page-link">{"..."}</span></li> }
                                    } else {
                                        let is_active = num == curr_page;
                                        html! {
                                            <li class={if is_active { "page-item active" } else { "page-item" }}>
                                                <a class="page-link" href="#"
                                                    onclick={
                                                        let current_page = current_page.clone();
                                                        Callback::from(move |_| current_page.set(num))}>
                                                    { num.to_string() }
                                                </a>
                                            </li>
                                        }
                                    }
                                })
                            }
                            <li class={if curr_page == tot_pages { "page-item disabled" } else { "page-item" }}>
                                <a class="page-link" href="#" onclick={
                                    let current_page = current_page.clone();
                                    Callback::from(move |_| current_page.set(usize::min(tot_pages, curr_page + 1)))
                                }>{"Next"}</a>
                            </li>
                        </ul>
                    </nav>
                }
            </>
        }
    }
}