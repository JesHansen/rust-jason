use serde::{Deserialize, Serialize};

use super::{Person, Pet};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum Resident {
    Owner(Person),
    Companion(Pet),
}
