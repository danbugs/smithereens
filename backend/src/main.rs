use rocket::http::Status;
use rocket::serde::json::Json;
use diesel::prelude::*;

use smithe_database::{db_models::player::Player, schema::players::dsl::*};

#[macro_use]
extern crate rocket;

#[get("/<gamer_tag>")]
pub fn search_players(gamer_tag: &str) -> Result<Json<Vec<Player>>, Status> {
    let processed_tag = gamer_tag.replace(' ', "%");
    // ^^^ transform spaces into wildcards to make search more inclusive

    let db_connection = smithe_database::connect().map_err(|_| Status::InternalServerError)?;
    Ok(Json(
        players
            .filter(gamer_tag_with_prefix.ilike(format!("%{}%", processed_tag))) // case-insensitive like
            .get_results::<Player>(&db_connection)
            .map_err(|_| Status::InternalServerError)?,
    ))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/players", routes![search_players])
}
