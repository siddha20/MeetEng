use super::*;

pub struct People {
    pub students: Vec<Student>,
    pub mentors: Vec<Mentor>
}

// weights for similarity metric
struct weights {
    availability: f64,
    major: f64,
    interests: f64,
    gender: f64,
    sports: f64, 
    rotc: f64
}

// for time we should use the l2 norm and f(x) = 1 - x/(x+100)