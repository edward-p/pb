#[derive(Responder)]
#[response(status = 418, content_type = "plain")]
pub struct PlainTeapot(pub &'static str);
