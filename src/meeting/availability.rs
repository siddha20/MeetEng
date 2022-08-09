use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Availability {
    pub year: u32, // xxxx
    pub month: u32, // xx
    pub day: u32, // xx
    pub time: u32 // xxxx (24hr)
}

impl Availability {
    pub fn new() -> Self {
        Availability {
            year: 0,
            month: 0,
            day: 0,
            time: 0,
        }
    }

    pub fn mag(&self) -> f64 {
        ((self.year as f64).powf(2.0) + (self.month as f64).powf(2.0) +
             (self.day as f64).powf(2.0) + (self.time as f64).powf(2.0)).sqrt()
    }
}

// cosine similarity
pub fn sim(a: &Availability, b:&Availability) -> f64 {
    ((a.year as f64 * b.year as f64) + (a.month as f64 * b.month as f64) + 
        (a.day as f64 * b.day as f64) + (a.time as f64 * b.time as f64))/
            (a.mag() * b.mag())
}