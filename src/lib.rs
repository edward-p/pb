use rocket::{
    data::ToByteUnit,
    form::{self, DataField, Error, FromFormField, ValueField},
};
use std::{
    collections::hash_map::DefaultHasher,
    env,
    hash::{Hash, Hasher},
    ops::Deref,
    str,
};

#[macro_use]
extern crate rocket;

#[derive(Debug, FromForm)]
pub struct Paste {
    #[field(name = "c")]
    #[field(name = "content")]
    #[field(name = "u")]
    #[field(name = "url")]
    pub content: Content,
}

#[derive(Debug)]
pub enum ContentValue {
    Bytes(Vec<u8>),
    Url(Vec<u8>),
}

#[derive(Debug)]
pub struct Content {
    pub value: ContentValue,
    pub hash: String,
}

impl Content {
    fn from(value: ContentValue) -> Content {
        const BASE64: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_=";
        const SIZE: usize = 5;

        // calculate hash
        use ContentValue::*;
        let value_vec = match &value {
            Bytes(bytes) | Url(bytes) => bytes,
        };
        let mut hasher = DefaultHasher::new();
        value_vec.hash(&mut hasher);
        let hash_u64 = hasher.finish();
        let mut hash_str = String::with_capacity(SIZE);

        // base64ify hash_u64
        for i in (1..SIZE + 1).rev() {
            let shift = i * 8;
            let v = (hash_u64 << (64 - shift) >> 56) as usize;
            hash_str.push(BASE64[v % 64] as char);
        }

        Content {
            value,
            hash: hash_str,
        }
    }
}

#[async_trait]
impl<'r> FromFormField<'r> for Content {
    fn from_value(_field: ValueField<'r>) -> form::Result<'r, Self> {
        Err((Error::validation("I'm a teapot.")).into())
    }

    async fn from_data(field: DataField<'r, '_>) -> form::Result<'r, Self> {
        let mut bytes: Vec<u8> = Vec::new();
        field
            .data
            .open(20.mebibytes())
            .stream_to(&mut bytes)
            .await
            .unwrap();

        let name = field.name.as_name().as_str();
        println!("{}", name);
        if name == "c" || name == "content" {
            // Content
            Ok(Content::from(ContentValue::Bytes(bytes)))
        } else if name == "u" || name == "url" {
            // Url
            let url = String::from_utf8(bytes).unwrap();
            if !url.starts_with("http://") && !url.starts_with("https://") {
                return Err((Error::validation("I'm a teapot.")).into());
            }

            Ok(Content::from(ContentValue::Url(url.into())))
        } else {
            Err((Error::validation("I'm a teapot.")).into())
        }
    }
}

#[derive(Responder)]
#[response(status = 418, content_type = "plain")]
pub struct PlainTeapot(pub &'static str);

pub struct PbConfig {
    pub pb_data: String,
    pub pb_site: String,
    pub index: String,
}

impl Deref for PbConfig {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.pb_data
    }
}

impl PbConfig {
    pub fn new() -> PbConfig {
        let pb_data = env::var("PB_DATA");
        let pb_data = match pb_data {
            Ok(path) => path,
            Err(_) => "./pb_data".into(),
        };
        let pb_site = env::var("PB_SITE");
        let pb_site = match pb_site {
            Ok(pb_site) => pb_site,
            Err(_) => "http://localhost:8000".into(),
        };

        let index = format!(
            "
    USAGE

      PASTE

          curl {} -F 'c=@-;type=*/*' < /path/to/file

      URL SHORTEN
  
          curl {} -F 'u=@-;type=*/*' <<< 'url_to_shorten'

      DELETE

          for content: curl -X DELETE {}/hex

          for url: curl -X DELETE {}/u/hex
    \n",
            pb_site, pb_site, pb_site, pb_site
        );

        PbConfig {
            pb_data,
            pb_site,
            index,
        }
    }
}
