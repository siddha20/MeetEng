use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Time {
    pub year: i32, // xxxx
    pub month: i32, // xx
    pub day: i32, // xx
    pub time: i32 // xxxx (24hr)
}

impl Time {
    pub fn new() -> Self {
        Time {
            year: 0,
            month: 0,
            day: 0,
            time: 0,
        }
    }

    pub fn mag(&self) -> f64 {
        ((self.year as f64).powf(2.0) +
         (self.month as f64).powf(2.0) +
         (self.day as f64).powf(2.0) +
         (self.time as f64).powf(2.0)).sqrt()
    }
}

// cosine similarity
pub fn cos_sim(a: &Time, b: &Time) -> f64 {
    ((a.year as f64 * b.year as f64) +
     (a.month as f64 * b.month as f64) + 
     (a.day as f64 * b.day as f64) +
     (a.time as f64 * b.time as f64))/
     (a.mag() * b.mag())
}

// L2 norm
pub fn l2(a: &Time, b: &Time) -> f64 {
    (((a.year - b.year) as f64 * 525600.0).powf(2.0) +
    ((a.month - b.month) as f64 * 43800.0).powf(2.0) +
    ((a.day - b.day) as f64 * 1440.0).powf(2.0) +
    ((a.time - b.time) as f64).powf(2.0)).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sim() {
        let a = Time{
            year: 2022,
            month: 08,
            day: 14,
            time: 1000
        };
        let b = Time{
            year: 2022,
            month: 08,
            day: 15,
            time: 1030
        };
        let res = l2(&a, &b);
        println!("result: {} scaled: {}", res, 1.0 - (res/(res + 100.0)));
        assert!(true);
    }
}