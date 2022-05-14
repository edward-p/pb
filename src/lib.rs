pub mod entity;

use std::{env, str};

use rocket::serde::{Deserialize, Serialize};

#[macro_use]
extern crate rocket;

#[derive(Responder)]
#[response(status = 418, content_type = "plain")]
pub struct PlainTeapot(pub &'static str);

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PbConfig {
    pub pb_data: String,
    pub pb_site: String,
    pub static_file_path: String,
}

impl Default for PbConfig {
    fn default() -> Self {
        let pb_data = match env::var("PB_DATA") {
            Ok(path) => path,
            Err(_) => "./pb_data".into(),
        };

        let pb_site = match env::var("PB_SITE") {
            Ok(pb_site) => pb_site,
            Err(_) => "http://localhost:8000".into(),
        };

        let static_file_path = match env::var("STATIC_FILE_PATH") {
            Ok(p) => p,
            Err(_) => "./static".into(),
        };

        Self {
            pb_data,
            pb_site,
            static_file_path,
        }
    }
}
