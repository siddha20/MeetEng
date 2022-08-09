use super::availability::Availability;
use serde::{Deserialize, Serialize};
use serde_json;


#[derive(Serialize, Deserialize)]
pub struct Student {
    pub name: String,
    pub major: String,
    pub availability: Availability,
    pub interests: Vec<String>,
    pub gender: String,
    pub sports: Vec<String>,
    pub rotc: bool,
}


#[derive(Serialize, Deserialize)]
pub struct Mentor {
    pub name: String,
    pub major: String,
    pub availability: Availability,
    pub interests: Vec<String>,
    pub gender: String,
    pub sports: Vec<String>,
    pub rotc: bool,
}


#[derive(Serialize, Deserialize)]
pub struct Admin {
    pub name: String, 
    pub account: String,
}

pub trait json {
    fn to_json_string(&self) -> String;
}

impl json for Student {
    fn to_json_string(&self) -> String {
        serde_json::to_string(&self).expect("serialize failed")
    }
}

impl json for Mentor {
    fn to_json_string(&self) -> String {
        serde_json::to_string(&self).expect("serialize failed")
    }
}

impl json for Admin {
    fn to_json_string(&self) -> String {
        serde_json::to_string(&self).expect("serialize failed")
    }
}