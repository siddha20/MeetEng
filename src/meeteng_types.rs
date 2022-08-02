pub mod people {

    pub struct Availability{
        pub test: i32,
    }

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
}



pub mod items {
    use super::people:: {
        Student,
        Mentor
    };

    pub struct Oauth2Details {
        pub client_id: String, 
        pub client_secret: String,
        pub scopes: Vec<String>,
        pub redirect_uri: String,
        pub auth_url: String,
        pub token_url: String
    }

    pub struct Meeting {
        pub date: String,
        pub mentor: Mentor,
        pub student: Student
        
    }

    pub struct FormAPI {
        pub end_point: String
    }    
}