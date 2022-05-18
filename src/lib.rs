pub mod catcher;
pub mod entity;
pub mod handler;
pub mod responder;

use std::str;

use rocket::serde::{Deserialize, Serialize};

#[macro_use]
extern crate rocket;

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PbConfig {
    pub pb_data: String,
    pub pb_site: String,
    pub static_file_path: String,
}

impl Default for PbConfig {
    fn default() -> Self {
        Self {
            pb_data: "./pb_data".into(),
            pb_site: "http://localhost:8000".into(),
            static_file_path: "./static".into(),
        }
    }
}
