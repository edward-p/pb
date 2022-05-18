use pb::*;
use rocket::fairing::AdHoc;
use rocket::figment::providers::{Env, Serialized};
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    let figment = rocket::Config::figment()
        .merge(
            Env::raw()
                .only(&["PB_DATA", "PB_SITE", "STATIC_FILE_PATH"])
                .global(),
        )
        .join(Serialized::defaults(PbConfig::default()))
        .join(("limits.data-form", "20 Mib"));

    let config: PbConfig = figment.extract().unwrap();

    use catcher::*;
    use handler::*;

    rocket::custom(figment)
        .attach(AdHoc::config::<PbConfig>())
        .attach(Template::fairing())
        .mount(
            "/",
            rocket::routes![
                retrieve_content,
                retrieve_url,
                syntax_highlighting,
                delete_url,
                delete_content,
                paste
            ],
        )
        .mount("/", FileServer::from(config.static_file_path))
        .register("/", rocket::catchers![not_found_catcher, default_catcher])
}
