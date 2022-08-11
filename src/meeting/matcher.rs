use super::*;

pub struct People {
    pub students: Vec<Student>,
    pub mentors: Vec<Mentor>
}

// weights for similarity metric
pub struct Weights {
    pub avail: f64,
    pub major: f64,
    pub interests: f64,
    pub gender: f64,
    pub sports: f64, 
    pub rotc: f64
}

pub struct Metrics {
    pub weights: Weights,
}

impl Weights {
    pub fn new() -> Self {
        Weights {
            avail: 1.0,
            major: 1.0,
            interests: 1.0,
            gender: 1.0,
            sports: 1.0,
            rotc: 1.0
        }
    }
}


impl Metrics {
    pub fn new() -> Self {
        Metrics {
            weights: Weights::new(),
        }
    }

    pub fn value(&self, a: &Student, b: &Mentor) -> f64 {
        (self.weights.avail * self.avail_sim(&a, &b)) +
        (self.weights.major * self.major_sim(&a, &b)) +
        (self.weights.interests * self.interests_sim(&a, &b)) +
        (self.weights.gender * self.gender_sim(&a, &b)) +
        (self.weights.sports * self.sports_sim(&a, &b)) +
        (self.weights.rotc * self.rotc_sim(&a, &b))
    }

    // for time we should use the l2 norm and f(x) = 1 - x/(x+100)
    fn avail_sim(&self, a: &Student, b: &Mentor) -> f64 {
        let res = Time::l2(&a.availability, &b.availability);
        (1.0 - res/(res + 100.0))
    }

    fn major_sim(&self, a: &Student, b: &Mentor) -> f64 {
        if (a.major.eq(&b.major)) {
            return 1.0;
        }
        0.0
    }

    fn interests_sim(&self, a: &Student, b: &Mentor) -> f64 {
        0.0
    }

    fn gender_sim(&self, a: &Student, b: &Mentor) -> f64 {
        if (a.gender.eq(&b.gender)) {
            return 1.0;
        }
        0.0
    }

    fn sports_sim(&self, a: &Student, b: &Mentor) -> f64 {
        0.0
    }

    fn rotc_sim(&self, a: &Student, b: &Mentor) -> f64 {
        if (a.rotc && b.rotc) {
            return 1.0;
        }
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metric_value() {
        assert!(true);
    }
}


