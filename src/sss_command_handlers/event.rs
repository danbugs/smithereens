use crate::queries::{
    event_getter::get_phase_id_from_event_slug, seeding_geeter::make_seeding_getter_query,
};
use anyhow::Result;
use url::Url;

pub async fn handle_event(url: Url) -> Result<()> {
    let slug = &url.path()[1..];
    let phase_id = get_phase_id_from_event_slug(slug).await?;
    dbg!(make_seeding_getter_query(phase_id).await?);
    Ok(())
}
