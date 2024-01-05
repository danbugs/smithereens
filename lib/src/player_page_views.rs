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

#[cfg(test)]
mod tests {
    #![allow(unused)]    
    use super::*;

    #[test]
    #[cfg(feature = "skip_db_tests")]
    fn test_insert_player_page_view() -> Result<()> {
        // get count of all player_page_views w/ -999 pid
        let mut db_connection = smithe_database::connect().unwrap();
        let ppv = player_page_views.filter(player_id.eq(-999));
        let count = ppv.count().get_result::<i64>(&mut db_connection).unwrap();

        // insert player_page_view
        let pid = -999;
        insert_player_page_view(pid)?;

        // get count again and check that it increased by 1
        let new_count = ppv.count().get_result::<i64>(&mut db_connection).unwrap();
        assert_eq!(new_count, count + 1);

        // delete all player_page_views w/ -999 pid
        diesel::delete(ppv).execute(&mut db_connection)?;

        Ok(())
    }
}
