use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use serde::Deserialize;
use std::error::Error;

pub fn read_json(file_name: &str) -> Result<Major, Box<dyn Error>> {
    let mut buff = String::new();
    File::open(file_name)?
        .read_to_string(&mut buff)?;

    match serde_json::from_str(&buff) {
        Ok(major) => Ok(major),
        Err(e) => Err(Box::new(e)),
    }
}

#[derive(Deserialize, Debug)]
pub struct Major {
    major_reqs: Req,
    all_reqs: HashMap<String, Req>,
    all_classes: HashMap<String, Class>,
}

#[derive(Deserialize, Debug, PartialEq, Eq, PartialOrd)]
pub struct Class {
    name: String,
    credits: i32,
    sems: Sems,
    prereqs: Req,
}

#[derive(Deserialize, Debug, PartialEq, Eq, PartialOrd)]
pub struct Req {
    reqs: Vec<String>,
    needed: i32,
    counts_for: Vec<String>,
}

#[derive(Deserialize, Debug, PartialEq, Eq, PartialOrd)]
struct Sems {
    fall: bool,
    spring: bool,
    summer: bool,
}
