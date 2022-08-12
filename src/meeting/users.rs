use super::time::Time;
use serde::{Deserialize, Serialize};

use serde_json;


#[derive(Serialize, Deserialize, Debug)]
pub struct Student {
    pub name: String,
    pub major: String,
    pub availability: Vec<Time>,
    pub interests: Vec<String>,
    pub gender: String,
    pub sports: Vec<String>,
    pub rotc: bool,
    pub matched: bool
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Mentor {
    pub name: String,
    pub major: String,
    pub availability: Vec<Time>,
    pub interests: Vec<String>,
    pub gender: String,
    pub sports: Vec<String>,
    pub rotc: bool,
    pub matched: bool
}

impl Mentor {
    pub fn new() -> Self {
        Mentor {
            name: "test".to_string(),
            major: "test".to_string(),
            availability: vec![Time::new()],
            interests: vec!["test".to_string()],
            gender: "test".to_string(),
            sports: vec!["test".to_string()],
            rotc: true,
            matched: false
        }
    }
}

impl Student {
    pub fn new() -> Self {
        Student {
            name: "test".to_string(),
            major: "test".to_string(),
            availability: vec![Time::new()],
            interests: vec!["test".to_string()],
            gender: "test".to_string(),
            sports: vec!["test".to_string()],
            rotc: true,
            matched: false
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Admin {
    pub name: String, 
    pub account: String
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sim() {
        // let mentor = Mentor::new();
        // let student = Student::new();
        // let res = sim(&student, &mentor);
        // println!("result: {}", res);
        assert!(true);
    }

    #[test]
    fn test_serialize() {
        let mentor = Mentor::new();
        let student = Student::new();
        let serialized = serde_json::to_string(&mentor).unwrap();
        println!("{}\n", serialized);
        println!("{:?}\n", mentor);
        assert!(true);
    }
}
