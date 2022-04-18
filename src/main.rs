use pb::*;
use rocket::form::Form;
use rocket::http::ContentType;
use rocket::response::Redirect;
use rocket::tokio::fs::{self, File};
use rocket::tokio::io::{self, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use rocket::State;
use rocket_dyn_templates::Template;
use std::collections::HashMap;
use std::path::Path;
use std::str::FromStr;

#[macro_use]
extern crate rocket;

#[catch(default)]
fn default_catcher() -> PlainTeapot {
    PlainTeapot("I'm a teapot. (┙>∧<)┙へ┻┻\n")
}

#[catch(404)]
fn not_found_catcher() -> &'static str {
    "404 (┙>∧<)┙へ┻┻\n"
}

#[post("/", data = "<paste>")]
async fn paste(pb_config: &State<PbConfig>, paste: Form<Paste>) -> io::Result<String> {
    let content = &paste.content;

    let bytes = match &content.value {
        ContentValue::Bytes(bytes) | ContentValue::Url(bytes) => bytes,
    };

    let dir = match &content.value {
        ContentValue::Bytes(_) => "content",
        ContentValue::Url(_) => "url",
    };

    let path = Path::new(&pb_config.pb_data)
        .join(dir)
        .join(content.hash.as_str());

    let mut url = format!("{}/", pb_config.pb_site);

    if let ContentValue::Url(_) = &content.value {
        url.push_str("u/")
    }

    url.push_str(content.hash.as_str());
    url.push('\n');

    if path.exists() {
        return Ok(format!("{}Already exist!\n", url));
    }

    let parent = path.parent().unwrap();
    if !parent.exists() {
        fs::create_dir_all(parent).await?;
    }

    let mut file = File::create(path).await?;
    file.write_all(bytes).await?;

    Ok(url)
}

#[get("/")]
fn index(pb_config: &State<PbConfig>) -> &str {
    pb_config.index.as_str()
}

#[get("/<id>")]
async fn retrieve_content(
    id: &str,
    pb_config: &State<PbConfig>,
) -> Option<(ContentType, Option<rocket::tokio::fs::File>)> {
    let path = Path::new(&pb_config.pb_data).join("content").join(id);

    if !path.exists() {
        return None;
    }

    let kind = infer::get_from_path(&path).expect("file read successfully");

    let content_type = match kind {
        Some(t) => ContentType::from_str(t.mime_type()).unwrap_or(ContentType::Plain),
        None => ContentType::Plain,
    };

    Some((content_type, rocket::tokio::fs::File::open(path).await.ok()))
}

#[get("/<id>/<lang>")]
async fn syntax_highlighting(
    id: &str,
    lang: &str,
    pb_config: &State<PbConfig>,
) -> io::Result<Option<Template>> {
    let mut context: HashMap<String, String> = HashMap::new();
    context.insert("tittle".into(), id.into());
    context.insert("lang".into(), lang.into());

    let path = Path::new(&pb_config.pb_data).join("content").join(id);

    if !path.exists() {
        return Ok(None);
    }

    let mut file = File::open(path).await?;
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).await?;
    let content = String::from_utf8(buffer).unwrap();
    context.insert("content".into(), content);

    Ok(Some(Template::render("syntax_highlighting", &context)))
}

#[get("/u/<id>")]
async fn retrieve_url(id: &str, pb_config: &State<PbConfig>) -> io::Result<Option<Redirect>> {
    let path = Path::new(&pb_config.pb_data).join("url").join(id);
    if !path.exists() {
        return Ok(None);
    }

    let mut reader = BufReader::new(File::open(path).await?);
    let mut line = String::new();
    reader.read_line(&mut line).await?;

    Ok(Some(Redirect::to(String::from(line.trim()))))
}

#[delete("/<id>")]
async fn delete_content(id: &str, pb_config: &State<PbConfig>) -> io::Result<&'static str> {
    fs::remove_file(Path::new(&pb_config.pb_data).join("content").join(id)).await?;
    Ok("Done!\n")
}

#[delete("/u/<id>")]
async fn delete_url(id: &str, pb_config: &State<PbConfig>) -> io::Result<&'static str> {
    fs::remove_file(Path::new(&pb_config.pb_data).join("url").join(id)).await?;
    Ok("Done!\n")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(PbConfig::new())
        .attach(Template::fairing())
        .mount(
            "/",
            rocket::routes![
                index,
                retrieve_content,
                retrieve_url,
                syntax_highlighting,
                delete_url,
                delete_content,
                paste
            ],
        )
        .register("/", rocket::catchers![not_found_catcher, default_catcher])
}
