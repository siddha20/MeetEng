mod users;
mod availability;

use users:: {
    Mentor,
    Student
};

use availability::Availability;

pub struct Meeting {
    pub date: String,
    pub mentor: Mentor,
    pub student: Student
}

impl Meeting {
    pub fn new() -> Self {
        let mentor = Mentor {
            name: "test".to_string(),
            major: "test".to_string(),
            availability: Availability {},
            interests: vec!["test".to_string()],
            gender: "test".to_string(),
            sports: vec!["test".to_string()],
            rotc: true,
        };
        let student = Student {
            name: "test".to_string(),
            major: "test".to_string(),
            availability: Availability {},
            interests: vec!["test".to_string()],
            gender: "test".to_string(),
            sports: vec!["test".to_string()],
            rotc: true,
        };
        Meeting {
            date: "date".to_string(),
            mentor: mentor,
            student: student,
        }
    }
    
    pub fn display(&self) {
        println!("{}, {}, {}", self.date, self.mentor.major, self.student.major);
    }
}