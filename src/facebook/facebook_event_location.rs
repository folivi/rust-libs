use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize, Clone, GraphQLObject)]
pub struct FacebookEventLocation {
    pub city:  Option<String>,
    pub country:  Option<String>,
    pub latitude:  Option<String>,
    pub longitude: Option<String>,
    pub street: Option<String>,
    pub zip: Option<String>,
    pub coords: Option<Vec<f64>>
}