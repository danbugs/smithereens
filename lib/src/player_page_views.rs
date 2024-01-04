use anyhow::Result;
use diesel::prelude::*;
use smithe_database::{
    db_models::player_page_views::NewPlayerPageView, schema::player_page_views::dsl::*,
};

pub fn insert_player_page_view(pid: i32) -> Result<()> {
    let new_player_page_view = NewPlayerPageView::new(pid);
    let mut db_connection = smithe_database::connect()?;

    diesel::insert_into(player_page_views)
        .values(&new_player_page_view)
        .execute(&mut db_connection)?;

    Ok(())
}
