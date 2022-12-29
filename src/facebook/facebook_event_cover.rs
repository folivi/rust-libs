use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize, Clone, GraphQLObject)]
pub struct FacebookEventCover {
    pub source: String,
    pub id: String,
}
