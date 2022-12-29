
use serde::Deserialize;

use super::{facebook_event_place::FacebookEventPlace, facebook_event_cover::FacebookEventCover};


#[derive(Deserialize, Debug, Clone)]
pub struct FacebookEvent {
    pub description: String,
    pub start_time: String,
    pub end_time: Option<String>,
    pub place: Option<FacebookEventPlace>,
    pub name: String,
    pub id: String,
    pub cover: FacebookEventCover
}
