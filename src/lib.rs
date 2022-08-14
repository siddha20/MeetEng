mod auth;
use auth::get_link;

use wasm_bindgen::prelude::*;



fn add(a: u32, b: u32) -> u32 {
    a + b
}


#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn auth_redirect() -> String {
    get_link()
}