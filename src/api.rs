mod forms;
mod sheets;

use forms::form_api;

pub fn gen_form() -> form_api {
    form_api {
        endpoint: "test...".to_string(),
    }
}