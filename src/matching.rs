use crate::meeteng_types::people:: {
    Student,
    Mentor,
    Availability
};

use crate::meeteng_types::items::Meeting;

pub fn metric(s: Student, m: Mentor) -> f32 {
    1.0
}

#[cfg(test)]
mod test {
    use super::*;    
    fn sample_Student() -> Student {
        Student {
            name: "Siddha K".to_string(),
            major: "CS".to_string(),
            availability: Availability{
                test: 1,
            },
            interests: vec!["Biking".to_string(), "".to_string()],
            gender: "boymode".to_string(),
            sports: vec!["Skiing".to_string(), "Tag".to_string()],
            rotc: false,
        }
    }

    fn sample_Mentor() -> Mentor {
        Mentor {
            name: "Aidan Face".to_string(),
            major: "CS".to_string(),
            availability: Availability{
                test: 1,
            },
            interests: vec!["Hollow Knight".to_string(), "Big Mobi".to_string()],
            gender: "boymode".to_string(),
            sports: vec!["Bannana Eating".to_string()],
            rotc: false,
        }
    }
    
    #[test]
    /*
     * This test ensures that the metric calculated between a student and mentor
     * is between 0 and 1 (inclusive). The metric repersents the similarity between
     * the mentor and student in terms of their categories.
     */
    fn test_metric() {
        let test_student = sample_Student();
        let test_mentor = sample_Mentor();
        assert!(0 <= metric(test_student, test_mentor) && metric(test_student, test_mentor) <= 1);
        assert!(true);
    }

    #[test]
    /*
     * This test ensures that the metric calculated between a student and mentor
     * is less than 1, but greather than or equal to 0. Since there are no matching
     * availability between the mentor and student, the metric should not be 1 which
     * implies a perfect match.
     */
    fn test_metric_no_avail() {
        let test_student = sample_Student();
        let test_mentor = sample_Mentor();
        assert!(0 <= metric(test_student, test_mentor) && metric(test_student, test_mentor) < 1);

    }
    
    #[test]
    /*
     * This test ensures that the metric calculated between a student and mentor
     * is less than 1, but greather than or equal to 0. Since there are no matching
     * majors between the mentor and student, the metric should not be 1 which
     * implies a perfect match.
     */
    fn test_metric_no_major_match() {
        //These have no matching major
        let test_student = sample_Student();
        let test_mentor = sample_Mentor();
        assert!(0 <= metric(test_student, test_mentor) && metric(test_student, test_mentor) < 1);

    }

    #[test]
    /*
     * This test ensures that the metric calculated between a student and mentor
     * is less than 1, but greather than or equal to 0. Since there are no matching
     * interests between the mentor and student, the metric should not be 1 which
     * implies a perfect match.
     */
    fn test_metric_no_interest_match() {
        //These have no interest overlap
        let test_student = sample_Student();
        let test_mentor = sample_Mentor();
        assert_eq!(0 <= metric(test_student, test_mentor) && metric(test_student, test_mentor) < 1);

    }

    #[test]
    /*
     * This test ensures that the metric is exactly equal to 1. The metric must
     * be equal to 1 because the student and mentor have perfect matches in all
     * categories. 
     */
    fn test_metric_full_match() {
        let test_student = 
        Student {
            name: "Siddha K".to_string(),
            major: "CS".to_string(),
            availability: Availability{
                test: 1,
            },
            interests: vec!["Biking".to_string(), "".to_string()],
            gender: "boymode".to_string(),
            sports: vec!["Skiing".to_string(), "Tag".to_string()],
            rotc: false,
        }
        let test_mentor = 
        Mentor {
            name: "Aidan Face".to_string(),
            major: "CS".to_string(),
            availability: Availability{
                test: 1,
            },
            interests: vec!["Biking".to_string(), "".to_string()],
            gender: "boymode".to_string(),
            sports: vec!["Skiing".to_string(), "Tag".to_string()],
            rotc: false,
        } 
        assert_eq!(metric(test_student, test_mentor) == 1);
    }
}
