use serde::{Deserialize, Serialize};

use super::Address;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Person {
    first_name: String,
    last_name: String,
    pub age: usize,
    address: Address,
    phone_numbers: Vec<String>,
}
