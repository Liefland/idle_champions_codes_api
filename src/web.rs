use crate::db;
use crate::db::Source;
use rocket::http::Status;
use rocket::response::Responder;
use rocket::serde::json::serde_json;
use rocket::Request;
use std::collections::HashMap;

#[derive(serde::Serialize)]
pub struct CodeResponse {
    pub code: String,
    pub expires_at: String,
    pub expired: bool,

    pub sources: SourceInformation,
}

#[derive(serde::Serialize)]
pub struct SourceInformation {
    pub lister: i32,
    pub submitter: i32,
    pub source: i32,
}

#[derive(serde::Serialize)]
pub struct ListCodesResponse {
    pub codes: Vec<CodeResponse>,
    pub sources: HashMap<i32, Source>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ErrorResponse {
    error: ErrorResponseInner,
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ErrorResponseInner {
    code: Status,
    description: String,
    debug: Option<String>,
}

#[derive(serde::Serialize)]
pub struct ConsumerRoute {
    pub(crate) method: &'static str,
    pub(crate) path: &'static str,
    pub(crate) description: &'static str,
}

impl ErrorResponse {
    pub fn new(code: Status, error: &str, debug: Option<String>) -> Self {
        ErrorResponse {
            error: ErrorResponseInner {
                code,
                description: error.to_string(),
                #[cfg(feature = "debug_errors")]
                debug,
                #[cfg(not(feature = "debug_errors"))]
                debug: None,
            },
        }
    }
}

impl<'r> Responder<'r, 'static> for ErrorResponse {
    fn respond_to(self, _: &Request) -> rocket::response::Result<'static> {
        let json = serde_json::to_string(&self).unwrap();

        rocket::Response::build()
            .status(self.error.code)
            .header(rocket::http::ContentType::JSON)
            .sized_body(json.len(), std::io::Cursor::new(json))
            .ok()
    }
}

impl ListCodesResponse {
    pub fn from(codes: Vec<db::FullCode>) -> Self {
        let mut sources = HashMap::new();

        let now_odt = time::OffsetDateTime::now_utc();
        let now_pdt = time::PrimitiveDateTime::new(now_odt.date(), now_odt.time());

        let codes = codes
            .into_iter()
            .map(|code| {
                sources.insert(code.lister.id.unwrap(), code.lister.clone());
                sources.insert(code.submitter.id.unwrap(), code.submitter.clone());
                sources.insert(code.creator.id.unwrap(), code.creator.clone());

                CodeResponse {
                    code: code.code.code,
                    expires_at: code.code.expires_at.to_string(),
                    expired: code.code.expires_at < now_pdt,
                    sources: SourceInformation {
                        lister: code.lister.id.unwrap(),
                        submitter: code.submitter.id.unwrap(),
                        source: code.creator.id.unwrap(),
                    },
                }
            })
            .collect();

        ListCodesResponse { codes, sources }
    }
}
