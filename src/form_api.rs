use std::collections::HashMap;
use crate::meeteng_types::items::FormAPI;
use crate::meeteng_types::people:: {
    Mentor,
    Student
};


impl FormAPI {
    
}

pub fn make_student_from_form(json: HashMap<&str, String>) -> Option<Student> {
    None
}

pub fn make_mentor_from_form(json: HashMap<&str, String>) -> Option<Mentor> {
    None
}


#[cfg(test)]
mod test {
    use super::*; 

    #[test]
    /*
     * This test asserts that a student was able to be extracted from JSON data
     * given that the JSON data was valid.
     */ 
    fn test_json_parsing_student(){
        let valid_student = getStudentJson();
        match make_student_from_form(valid_student) {
            None => assert!(false),
            _ => assert!(true),
        }
    }

    #[test]
    /*
     * This test asserts that a student was not able to be extracted from JSON
     *  data given that the JSON data was invalid.
     */ 
    fn test_json_parsing_student_error(){
        let invalid_student = getStudentJson();
        match make_student_from_form(valid_student) {
            None => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    /*
     * This test asserts that a mentor was able to be extracted from JSON data
     * given that the JSON data was valid.
     */ 
    fn test_json_parsing_mentor(){
        let valid_mentor = getMentorJson();
        match make_mentor_from_form(valid_mentor) {
            None => assert!(false),
            _ => assert!(true),
        }
    }
    
    #[test]
    /*
     * This test asserts that a mentor was not able to be extracted from JSON
     *  data given that the JSON data was invalid.
     */ 
    fn test_json_parsing_mentor_error(){
        let invalid_mentor = getMentorJson();
        match make_mentor_from_form(valid_mentor) {
            None => assert!(true),
            _ => assert!(false),
        }
    }
}