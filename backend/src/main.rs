#[macro_use]
extern crate rocket;

use rocket::{
    http::{Method, Status},
    response::{self, Responder},
    Build, Request, Rocket,
};
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use smithe_lib::{
    player::{get_all_like, get_player, get_top_two_characters},
    set::{
        get_competitor_type, get_set_losses_by_dq, get_set_losses_without_dqs, get_set_wins_by_dq,
        get_set_wins_without_dqs, get_sets_per_player_id, get_winrate, get_head_to_head_record,
    },
    tournament::get_tournaments_from_requester_id,
};
use thiserror::Error;

pub const DEV_ADDRESS: &str = "http://localhost:8080/";
pub const DEV_ADDRESS_2: &str = "http://127.0.0.1:8080/";

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
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<tag>")]
fn search_players(tag: String) -> Result<String, Error> {
    Ok(serde_json::to_string(&get_all_like(&tag)?)?)
}

#[get("/<id>")]
fn view_player(id: i32) -> Result<String, Error> {
    // insert player page view
    smithe_lib::player_page_views::insert_player_page_view(id).unwrap();
    Ok(serde_json::to_string(&get_player(id)?)?)
}

#[get("/<id>")]
async fn get_player_tournaments(id: i32) -> Result<String, Error> {
    Ok(serde_json::to_string(
        &get_tournaments_from_requester_id(id).await?,
    )?)
}

#[get("/<id>/wins_without_dqs")]
async fn get_player_set_wins_without_dqs(id: i32) -> Result<String, Error> {
    Ok(serde_json::to_string(&get_set_wins_without_dqs(id)?)?)
}

#[get("/<id>/losses_without_dqs")]
async fn get_player_set_losses_without_dqs(id: i32) -> Result<String, Error> {
    Ok(serde_json::to_string(&get_set_losses_without_dqs(id)?)?)
}

#[get("/<id>/wins_by_dqs")]
async fn get_player_set_wins_by_dqs(id: i32) -> Result<String, Error> {
    Ok(serde_json::to_string(&get_set_wins_by_dq(id)?)?)
}

#[get("/<id>/losses_by_dqs")]
async fn get_player_set_losses_by_dqs(id: i32) -> Result<String, Error> {
    Ok(serde_json::to_string(&get_set_losses_by_dq(id)?)?)
}

#[get("/<id>/winrate")]
async fn get_player_winrate(id: i32) -> Result<String, Error> {
    Ok(serde_json::to_string(&get_winrate(id)?)?)
}

#[get("/<id>/competitor_type")]
async fn get_player_competitor_type(id: i32) -> Result<String, Error> {
    let ct = get_competitor_type(id)?;
    Ok(serde_json::to_string(&format!("{}-{}er", ct.0, ct.1))?)
}

// endpoint to get_top_two_characters
#[get("/<id>/top_two_characters")]
async fn get_player_top_two_characters(id: i32) -> Result<String, Error> {
    Ok(serde_json::to_string(&get_top_two_characters(id)?)?)
}

// get sets by player id
#[get("/<id>")]
async fn get_player_sets(id: i32) -> Result<String, Error> {
    Ok(serde_json::to_string(&get_sets_per_player_id(id)?)?)
}

// get head to head by player id
#[get("/<id>/head_to_head")]
async fn get_player_head_to_head(id: i32) -> Result<String, Error> {
    Ok(serde_json::to_string(&get_head_to_head_record(id)?)?)
}

fn rocket() -> Rocket<Build> {
    let allowed_origins = AllowedOrigins::some_exact(&[DEV_ADDRESS, DEV_ADDRESS_2]);

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
            routes![view_player, get_player_top_two_characters, get_player_head_to_head],
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
        .attach(cors)
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = rocket().launch().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use rocket::{http::Status, local::blocking::Client};

    use super::rocket;

    #[test]
    fn get_player_tournaments_test() -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::tracked(rocket())?;
        let response = client.get("/tournaments/1178271").dispatch();
        assert_eq!(response.status(), Status::Ok);

        Ok(())
    }
}
