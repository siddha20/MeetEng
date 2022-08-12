use super::*;

pub struct People {
    pub students: Vec<Student>,
    pub mentors: Vec<Mentor>
}

/* Weights for similarity metric.
 * Should total to 1.0.
 */
pub struct Weights {
    pub avail: f64,
    pub major: f64,
    pub interests: f64,
    pub gender: f64,
    pub sports: f64, 
    pub rotc: f64
}

pub struct Metrics {
    pub weights: Weights
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

    // get the best matched time
    fn avail_sim(&self, a: &Student, b: &Mentor) -> f64 {
        let mut best_match: f64 = 0.0;
        for a_time in &a.availability{
            for b_time in &b.availability {
                let res = Time::l2(a_time, b_time);
                if (res > best_match) {
                    best_match = res;
                }
            }
        }
        best_match
    }

    fn major_sim(&self, a: &Student, b: &Mentor) -> f64 {
        if (a.major.eq(&b.major)) {
            return 1.0;
        }
        0.0
    }

    // check if at least one interest matches
    fn interests_sim(&self, a: &Student, b: &Mentor) -> f64 {
        for a_interest in &a.interests {
            for b_interest in &b.interests {
                if (a_interest.eq(b_interest)) {
                    return 1.0
                }
            }
        }
        0.0
    }

    fn gender_sim(&self, a: &Student, b: &Mentor) -> f64 {
        if (a.gender.eq(&b.gender)) {
            return 1.0;
        }
        0.0
    }

    // check if at least one sport matches 
    fn sports_sim(&self, a: &Student, b: &Mentor) -> f64 {
        for a_sport in &a.sports {
            for b_sport in &b.sports {
                if (a_sport.eq(b_sport)) {
                    return 1.0
                }
            }
        }
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


