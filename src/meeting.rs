mod users;
mod time;
pub mod matcher;

use users:: {
    Mentor,
    Student,
    json
};

use time::Time;

pub struct Meeting {
    pub date: String,
    pub mentor: Mentor,
    pub student: Student
}

impl Meeting {
    pub fn new() -> Self {
        let mentor = Mentor::new();
        let student = Student::new();
        Meeting {
            date: "date".to_string(),
            mentor: mentor,
            student: student
        }
    }

    pub fn display(&self) {
        let student_json = self.mentor.to_json_string();
        println!("the string: {}", student_json);
        let student_struct: Student = json::from_json(&student_json);
        println!("string to struct to string: {}", student_struct.to_json_string());
        assert!(student_json == student_struct.to_json_string());
        println!("{}, {}, {}", self.date, self.mentor.to_json_string(), self.student.to_json_string());
    }
}
