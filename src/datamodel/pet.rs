use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Pet {
    pub name: String,
    favorite_toy: String,
    eats: String,
}
