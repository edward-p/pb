use crate::responder::PlainTeapot;

#[catch(default)]
pub fn default_catcher() -> PlainTeapot {
    PlainTeapot("I'm a teapot. (┙>∧<)┙へ┻┻\n")
}

#[catch(404)]
pub fn not_found_catcher() -> &'static str {
    "404 (┙>∧<)┙へ┻┻\n"
}
