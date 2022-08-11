use super::time::Time;
use serde::{Deserialize, Serialize};
use serde_json;


#[derive(Serialize, Deserialize)]
pub struct Student {
    pub name: String,
    pub major: String,
    pub availability: Time,
    pub interests: Vec<String>,
    pub gender: String,
    pub sports: Vec<String>,
    pub rotc: bool
}


#[derive(Serialize, Deserialize)]
pub struct Mentor {
    pub name: String,
    pub major: String,
    pub availability: Time,
    pub interests: Vec<String>,
    pub gender: String,
    pub sports: Vec<String>,
    pub rotc: bool
}


#[derive(Serialize, Deserialize)]
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

// figure out what to do here: maybe this goes in matcher.
pub fn sim(student: &Student, mentor: &Mentor) -> f64 {
    09.0
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sim() {
        let mentor = Mentor {
            name: "test".to_string(),
            major: "test".to_string(),
            availability: Time::new(),
            interests: vec!["test".to_string()],
            gender: "test".to_string(),
            sports: vec!["test".to_string()],
            rotc: true,
        };
        let student = Student {
            name: "test".to_string(),
            major: "test".to_string(),
            availability: Time::new(),
            interests: vec!["test".to_string()],
            gender: "test".to_string(),
            sports: vec!["test".to_string()],
            rotc: true,
        };
        let res = sim(&student, &mentor);
        println!("result: {}", res);
        assert!(true);
    }
}