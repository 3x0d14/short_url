use crate::schema::urls;
use serde::{Deserialize, Serialize};
#[derive(Queryable, Serialize, Deserialize)]
pub struct Url {
    pub origin: String,
    pub destination: String,
}

#[derive(Insertable)]
#[table_name = "urls"]
pub struct NewUrl {
    pub origin: String,
    pub destination: String,
}
