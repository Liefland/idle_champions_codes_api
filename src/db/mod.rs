use time::Date;

mod api_key;
mod code;
mod source;

pub use api_key::*;
pub use code::*;

#[derive(Debug, serde::Serialize)]
pub struct Source {
    pub id: Option<i32>,
    pub name: String,
    pub url: String,
}

#[derive(Debug)]
pub struct ApiKey {
    pub id: Option<i32>,
    api_key: String,
    pub expired: bool,
    pub source_id: i32,
}

#[derive(Debug)]
pub struct Code {
    pub id: Option<i32>,
    pub code: String,
    pub expires_at: Date,
    pub submitter_id: i32,
    pub creator_id: i32,
    pub lister_id: i32,
}

#[derive(Debug)]
pub struct FullCode {
    pub code: Code,
    pub lister: Source,
    pub submitter: Source,
    pub creator: Source,
}

impl Clone for Source {
    fn clone(&self) -> Self {
        Source {
            id: self.id,
            name: self.name.clone(),
            url: self.url.clone(),
        }
    }
}
