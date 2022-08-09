use super::Availability;

pub struct Student {
    pub name: String,
    pub major: String,
    pub availability: Availability,
    pub interests: Vec<String>,
    pub gender: String,
    pub sports: Vec<String>,
    pub rotc: bool,
}

pub struct Mentor {
    pub name: String,
    pub major: String,
    pub availability: Availability,
    pub interests: Vec<String>,
    pub gender: String,
    pub sports: Vec<String>,
    pub rotc: bool,
}

pub struct Admin {
    pub name: String, 
    pub account: String,
}