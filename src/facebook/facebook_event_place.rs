use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

use super::facebook_event_location::FacebookEventLocation;


#[derive(Deserialize, Debug, Serialize, Clone, GraphQLObject)]

pub struct FacebookEventPlace {
    pub id: Option<String>,
    pub name: Option<String>,
    pub location: Option<FacebookEventLocation>,

}