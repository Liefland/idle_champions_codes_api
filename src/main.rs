#![allow(dead_code, unused_variables)]

mod auth;
mod db;
mod util;
mod web;

#[macro_use]
extern crate rocket;

use anyhow::Result;
use rocket::http::Status;

use crate::web::{ConsumerRoute, ErrorResponse, ListCodesResponse};
use rocket::serde::json::Json;
use rocket::time;

use crate::auth::ApiKey;
use rocket_db_pools::{sqlx, Connection, Database, Pool};
use sqlx::{Acquire, PgConnection};

#[derive(Database)]
#[database("postgres")]
pub struct Pg(sqlx::PgPool);

#[rocket::main]
async fn main() -> Result<()> {
    #[cfg(feature = "migrate")]
    sqlx::migrate!("db/migrations");

    rocket::build()
        .attach(Pg::init())
        .mount("/", routes![index, list_codes, insert_code])
        .launch()
        .await?;

    Ok(())
}

#[derive(serde::Serialize)]
pub struct IndexResponse {
    message: &'static str,
    source_url: &'static str,
    routes: Vec<ConsumerRoute>,
}

#[get("/")]
pub async fn index() -> Json<IndexResponse> {
    let routes = vec![
        ConsumerRoute {
            method: "GET",
            path: "/v1/codes?expired=false",
            description: "List all currently active codes",
        },
        ConsumerRoute {
            method: "GET",
            path: "/v1/codes?expired=true",
            description: "List all currently codes, allowing expired codes too",
        },
    ];
    Json(IndexResponse {
        message: "Welcome to a simple API for listing Idle Champions Codes!
This tool is meant to be used with my other idle champions tooling.
You will probably want to visit `/v1/codes`",
        source_url: "https://github.com/Zarthus/idle_champions_codes",
        routes,
    })
}

#[get("/v1/codes?<expired>")]
pub async fn list_codes(
    db: Connection<Pg>,
    expired: bool,
) -> Result<Json<ListCodesResponse>, ErrorResponse> {
    let codes = db::find_codes(db, expired).await;

    match codes {
        Ok(codes) => Ok(Json(ListCodesResponse::from(codes))),
        Err(e) => Err(ErrorResponse::new(
            Status::InternalServerError,
            "Internal Server Error",
            Some(e.to_string()),
        )),
    }
}

/// A partial code contains all the necessary fields to create a code in the database.
#[derive(serde::Deserialize, Debug)]
pub struct PartialCode {
    /// The code itself, should be 12 or 16 characters long - minus the dashes.
    pub code: String,
    /// The unix timestamp of when the code expires.
    pub expires_at: i64,
    /// The name of the creator - this is the person who created the code or first surfaced it (e.g. a streamer).
    pub creator_name: String,
    /// The url of the creator - this is the person who created the code or first surfaced it (e.g. a streamer).
    pub creator_url: String,
    /// The name of the submitter - this is the person who submitted the code to the source the lister is obtaining from.
    /// This can be identical to the creator if the submitter is the creatoer, in which case you can omit this field.
    pub submitter_name: Option<String>,
    /// The url of the submitter - this is the person who submitted the code to the source the lister is obtaining from.
    /// This can be identical to the creator if the submitter is the creatoer, in which case you can omit this field.
    pub submitter_url: Option<String>,
}

macro_rules! get_from_pool {
    ($pool:ident) => {
        $pool.acquire().await.map_err(|e| {
            ErrorResponse::new(
                Status::InternalServerError,
                "Failed to get connection from pool",
                Some(e.to_string()),
            )
        })?
    };
}

#[put("/v1/codes", data = "<partial_code>")]
pub async fn insert_code<'a>(
    pool: &Pg,
    token: ApiKey<'a>,
    partial_code: Json<PartialCode>,
) -> Result<Json<i32>, ErrorResponse> {
    let mut pool_connection = pool.get().await.map_err(|e| {
        ErrorResponse::new(
            Status::InternalServerError,
            "Failed to get connection from pool",
            Some(e.to_string()),
        )
    })?;

    let lister_source_id = ensure_valid_auth(get_from_pool!(pool_connection), token.get()).await?;

    let id = db::insert_partial_code(
        get_from_pool!(pool_connection),
        lister_source_id,
        partial_code.into_inner(),
    )
    .await
    .map_err(|e| {
        let errmsg = format!(
            "Failed to store code: {}",
            if e.to_string().contains("duplicate key") {
                "Duplicate code"
            } else {
                "Database Validation Failed"
            }
        );

        ErrorResponse::new(Status::BadRequest, &errmsg, Some(e.to_string()))
    })?;

    Ok(Json(id))
}

async fn ensure_valid_auth(conn: &mut PgConnection, token: &str) -> Result<i32, ErrorResponse> {
    match db::is_valid_api_key(conn, token).await {
        None => Err(ErrorResponse::new(
            Status::Unauthorized,
            "Invalid API key",
            None,
        )),
        Some(id) => Ok(id),
    }
}

impl PartialCode {
    pub fn expires_at_to_pdt(&self) -> Result<time::PrimitiveDateTime, time::Error> {
        util::time_parse_unix(self.expires_at)
    }
}
