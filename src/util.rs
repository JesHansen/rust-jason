use crate::datamodel::Resident;
use std::{fs, io::Read};

pub fn read_stringdata(path: &str) -> String {
    let mut f = fs::File::open(path).expect("file open failed");
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).expect("file read failed");
    buffer
}

pub fn deserialize_residents(data: &String) -> Vec<Resident> {
    let res = serde_json::from_str(data).expect("Bad day af the office, guys");
    res
}

#[inline]
pub fn map_residents(residents: &Vec<Resident>) -> usize {
    residents
        .iter()
        .map(|r: &Resident| match r {
            Resident::Owner(o) => o.age,
            Resident::Companion(c) => c.name.len(),
        })
        .sum()
}
