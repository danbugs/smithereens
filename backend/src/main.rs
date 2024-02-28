#![allow(non_snake_case)]
#[macro_use]
extern crate rocket;

use rocket::{
    http::{Method, Status},
    response::{self, Responder},
    Build, Request, Rocket,
};
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use rocket_governor::{Quota, RocketGovernable, RocketGovernor};
use smithe_lib::{
    player::{add_new_player_to_pidgtm_db, get_all_like, get_player, get_top_two_characters},
    set::{
        get_competitor_type, get_head_to_head_record, get_set_losses_by_dq,
        get_set_losses_without_dqs, get_set_wins_by_dq, get_set_wins_without_dqs,
        get_sets_per_player_id, get_winrate,
    },
    tournament::get_tournaments_from_requester_id,
};
use thiserror::Error;

use startgg::queries::player_getter::{make_pidgtm_player_getter_query, PIDGTM_PlayerGetterVars};

use std::sync::{Arc, Mutex};

pub const DEV_ADDRESS: &str = "http://localhost:8080/";
pub const DEV_ADDRESS_2: &str = "http://127.0.0.1:8080/";
pub const PROD_ADDRESS: &str = "http://smithe.net";
pub const PROD_ADDRESS_2: &str = "https://smithe.net";

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    SmitheLib(#[from] anyhow::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
}

impl<'r, 'o: 'r> Responder<'r, 'o> for Error {
    fn respond_to(self, req: &Request<'_>) -> response::Result<'o> {
        // todo: use open telemetry at this point
        Status::InternalServerError.respond_to(req)
    }
}

#[get("/")]
fn index(_limitguard: RocketGovernor<'_, RateLimitGuard>) -> &'static str {
    "Hello, world! (backend)"
}

#[get("/<tag>")]
async fn search_players(
    tag: String,
    _limitguard: RocketGovernor<'_, RateLimitGuard>,
) -> Result<String, Error> {
    Ok(serde_json::to_string(&get_all_like(&tag).await?)?)
}

#[get("/<id>")]
async fn view_player(
    id: i32,
    _limitguard: RocketGovernor<'_, RateLimitGuard>,
) -> Result<String, Error> {
    //Try to update if can but still serve old data otherwise
    if let Ok(player_data) =
        make_pidgtm_player_getter_query(id, Arc::new(Mutex::new(PIDGTM_PlayerGetterVars::empty())))
            .await
    {
        add_new_player_to_pidgtm_db(&player_data.player.unwrap()).await?;
    }

    // insert player page view
    smithe_lib::player_page_views::insert_player_page_view(id)
        .await
        .unwrap();
    Ok(serde_json::to_string(&get_player(id).await?)?)
}

#[get("/<id>")]
async fn get_player_tournaments(
    id: i32,
    _limitguard: RocketGovernor<'_, RateLimitGuard>,
) -> Result<String, Error> {
    Ok(serde_json::to_string(
        &get_tournaments_from_requester_id(id).await?,
    )?)
}

#[get("/<id>/wins_without_dqs")]
async fn get_player_set_wins_without_dqs(
    id: i32,
    _limitguard: RocketGovernor<'_, RateLimitGuard>,
) -> Result<String, Error> {
    Ok(serde_json::to_string(&get_set_wins_without_dqs(id).await?)?)
}

#[get("/<id>/losses_without_dqs")]
async fn get_player_set_losses_without_dqs(
    id: i32,
    _limitguard: RocketGovernor<'_, RateLimitGuard>,
) -> Result<String, Error> {
    Ok(serde_json::to_string(
        &get_set_losses_without_dqs(id).await?,
    )?)
}

#[get("/<id>/wins_by_dqs")]
async fn get_player_set_wins_by_dqs(
    id: i32,
    _limitguard: RocketGovernor<'_, RateLimitGuard>,
) -> Result<String, Error> {
    Ok(serde_json::to_string(&get_set_wins_by_dq(id).await?)?)
}

#[get("/<id>/losses_by_dqs")]
async fn get_player_set_losses_by_dqs(
    id: i32,
    _limitguard: RocketGovernor<'_, RateLimitGuard>,
) -> Result<String, Error> {
    Ok(serde_json::to_string(&get_set_losses_by_dq(id).await?)?)
}

#[get("/<id>/winrate")]
async fn get_player_winrate(
    id: i32,
    _limitguard: RocketGovernor<'_, RateLimitGuard>,
) -> Result<String, Error> {
    Ok(serde_json::to_string(&get_winrate(id).await?)?)
}

#[get("/<id>/competitor_type")]
async fn get_player_competitor_type(
    id: i32,
    _limitguard: RocketGovernor<'_, RateLimitGuard>,
) -> Result<String, Error> {
    let ct = get_competitor_type(id).await?;
    Ok(serde_json::to_string(&format!("{}-{}er", ct.0, ct.1))?)
}

// endpoint to get_top_two_characters
#[get("/<id>/top_two_characters")]
async fn get_player_top_two_characters(
    id: i32,
    _limitguard: RocketGovernor<'_, RateLimitGuard>,
) -> Result<String, Error> {
    Ok(serde_json::to_string(&get_top_two_characters(id).await?)?)
}

// get sets by player id
#[get("/<id>")]
async fn get_player_sets(
    id: i32,
    _limitguard: RocketGovernor<'_, RateLimitGuard>,
) -> Result<String, Error> {
    Ok(serde_json::to_string(&get_sets_per_player_id(id).await?)?)
}

// get head to head by player id
#[get("/<id>/head_to_head")]
async fn get_player_head_to_head(
    id: i32,
    _limitguard: RocketGovernor<'_, RateLimitGuard>,
) -> Result<String, Error> {
    Ok(serde_json::to_string(&get_head_to_head_record(id).await?)?)
}

fn rocket() -> Rocket<Build> {
    // #[cfg(debug_assertions)]
    // let allowed_origins = AllowedOrigins::some_exact(&[DEV_ADDRESS, DEV_ADDRESS_2]);

    // #[cfg(not(debug_assertions))]
    let allowed_origins = AllowedOrigins::some_exact(&[PROD_ADDRESS, PROD_ADDRESS_2]);

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Delete, Method::Put]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("failed to set cors");

    rocket::build()
        .mount("/", routes![index])
        .mount("/players", routes![search_players])
        .mount(
            "/player",
            routes![
                view_player,
                get_player_top_two_characters,
                get_player_head_to_head
            ],
        )
        .mount("/tournaments", routes![get_player_tournaments])
        .mount(
            "/sets",
            routes![
                get_player_sets,
                get_player_set_wins_without_dqs,
                get_player_set_losses_without_dqs,
                get_player_set_wins_by_dqs,
                get_player_set_losses_by_dqs,
                get_player_winrate,
                get_player_competitor_type,
            ],
        )
        .register("/", catchers!(rocket_governor::rocket_governor_catcher))
        .attach(cors)
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = rocket().launch().await?;
    Ok(())
}

pub struct RateLimitGuard;

impl<'r> RocketGovernable<'r> for RateLimitGuard {
    fn quota(_method: Method, _route_name: &str) -> Quota {
        Quota::per_minute(Self::nonzero(60u32))
    }
}
