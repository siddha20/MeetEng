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
    fn from_json(data: &String) -> Self;
}

impl json for Student {
    fn to_json_string(&self) -> String {
        serde_json::to_string(&self).expect("serialize failed")
    }
    fn from_json(data: &String) -> Student {
        let s: Student = serde_json::from_str(data).expect("deserialize failed");
        s
    }
}

impl json for Mentor {
    fn to_json_string(&self) -> String {
        serde_json::to_string(&self).expect("serialize failed")
    }
    fn from_json(data: &String) -> Mentor {
        let m: Mentor = serde_json::from_str(data).expect("deserialize failed");
        m
    }
}

impl json for Admin {
    fn to_json_string(&self) -> String {
        serde_json::to_string(&self).expect("serialize failed")
    }
    fn from_json(data: &String) -> Admin {
        let a: Admin = serde_json::from_str(data).expect("deserialize failed");
        a
    }
}