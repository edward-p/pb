use rocket::{
    data::ToByteUnit,
    form::{self, DataField, Error, FromFormField, ValueField},
};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

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

const BASE63: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_";
const SIZE: usize = 5;

impl From<ContentValue> for Content {
    fn from(value: ContentValue) -> Content {
        // calculate hash
        use ContentValue::*;
        let value_vec = match &value {
            Bytes(bytes) | Url(bytes) => bytes,
        };
        let mut hasher = DefaultHasher::new();
        value_vec.hash(&mut hasher);
        let hash_u64 = hasher.finish();
        let mut hash_str = String::with_capacity(SIZE);

        // baseXXify hash_u64
        for i in (1..SIZE + 1).rev() {
            let shift = i * 8;
            let v = (hash_u64 << (64 - shift) >> 56) as usize;
            hash_str.push(BASE63[v % 63] as char);
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
            .await?;

        use ContentValue::*;
        match field.name.as_name().as_str() {
            "c" | "content" => Ok(Bytes(bytes).into()),
            "u" | "url" if &bytes[0..7] == b"http://" || &bytes[0..8] == b"https://" => {
                Ok(Url(bytes).into())
            }
            _ => Err((Error::validation("I'm a teapot.")).into()),
        }
    }
}
