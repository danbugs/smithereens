use country_emoji::flag;
use yew::{function_component, html, Html, Properties};

use crate::components::loading_spinner::LoadingSpinner;
use crate::models::Player;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub selected_player: Option<Player>,
    pub selected_player_summary_data: Option<(String, String, String, String, Vec<String>)>,
    pub display: bool,
}

#[function_component(PlayerProfileHeader)]
pub fn player_profile_header(props: &Props) -> Html {
    if props.selected_player.is_some() {
        let selected_player = props.selected_player.clone().unwrap();
        html! {
            // display image and gamer tag side by side
            <div class="container">
                <br/>
                <br/>
                <div class="row">
                    // align profile picture in center of row
                    <div class="col-md-2 col-6 align-self-center">
                        <img src={
                            if selected_player.profile_picture.clone().is_some() && !selected_player.profile_picture.clone().unwrap().is_empty() {
                                selected_player.profile_picture.clone().unwrap()
                            } else {
                                "https://i.imgur.com/SwpJ1YU.png".to_string()
                            }
                        } alt="Profile Picture" class="img-fluid rounded-circle" referrerpolicy="no-referrer" style="width:128px;height:128px;"/>
                    </div>
                    <div class="col-md-2 col-6">
                        <h2>
                        {
                            if selected_player.prefix.is_none() || selected_player.prefix.as_ref().unwrap().is_empty() {
                                selected_player.gamer_tag.to_string()
                            } else {
                                format!("{} | {}", selected_player.prefix.as_ref().unwrap(), &selected_player.gamer_tag)
                            }
                        }
                        </h2>
                        // display name if it exists, otherwise nothing
                        {
                            if selected_player.name.is_some() && !selected_player.name.as_ref().unwrap().is_empty() {
                                html! {
                                    // display name and flag in a p in grey
                                    <p class="text-muted font-weight-light">{
                                        format!("{} {}",
                                            selected_player.name.as_ref().unwrap(),
                                            if let Some(pc) = selected_player.country.clone() {
                                                flag(&pc).unwrap_or("".to_string())
                                            } else {
                                                "".to_string()
                                            })}
                                    </p>
                                }
                            } else {
                                html! {
                                    <div></div>
                                }
                            }
                        }

                        // display country in a p element if it is some, otherwise nothing
                        {if selected_player.gender_pronouns.is_some() {
                        html! {
                            <p class="text-muted font-weight-light">{&selected_player.gender_pronouns.as_ref().unwrap()}</p>
                        }
                        } else {
                            html! {
                                <div></div>
                            }
                        }}

                        // display link to twitter if it is some, otherwise nothing
                        {if selected_player.twitter_username.is_some() {
                            html! {
                            <a target="_blank" href={format!("https://twitter.com/{}", selected_player.twitter_username.as_ref().unwrap_or(&String::from("")))}>
                                // img twitter dark icon 32px
                                <img src="https://abs.twimg.com/favicons/twitter.ico" alt="Twitter" class="img-fluid" style="max-width:32px"/>
                            </a>
                            }
                        } else {
                            html! {
                                <div></div>
                            }
                        }}

                        // display link to twitter if it is some, otherwise nothing
                        {
                            if selected_player.twitch_username.is_some() {
                                html! {
                                    <a target="_blank" href={format!("https://twitch.tv/{}", selected_player.twitch_username.as_ref().unwrap_or(&String::from("")))}>
                                        <img src="https://static.twitchcdn.net/assets/favicon-32-d6025c14e900565d6177.png" alt="Twitch" class="img-fluid" style="max-width:32px"/>
                                    </a>
                                }
                            } else {
                                html! {
                                    <div></div>
                                }
                            }
                        }
                    </div>

                    <div class="col-md-4 col-12 offset-md-4">
                        <br/>
                        {
                            if props.display {
                                html! {
                                    <div class="row justify-content-end">
                                        {
                                            if props.selected_player_summary_data.as_ref().unwrap().4.is_empty() {
                                                html! {
                                                    <div></div>
                                                }
                                            } else {
                                                props.selected_player_summary_data.as_ref().unwrap().4.iter().map(|character| {
                                                    html! {
                                                        <div class="col-auto">
                                                            <img src={format!("/assets/character_images/{} (Small).png", character)} alt="Character" class="img-fluid" style="max-width:128px;max-height:128px;"/>
                                                        </div>
                                                    }
                                                }).collect::<Html>()
                                            }
                                        }
                                    </div>
                                }
                            } else {
                                html! {
                                    <LoadingSpinner/>
                                }
                            }
                        }
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
