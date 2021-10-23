mod datamodel;
pub mod util;

use util::*;

pub fn kanulja() -> usize {
    let string_data = read_stringdata("LillyAndI.json");
    let residents = deserialize_residents(&string_data);
    map_residents(&residents)
}
