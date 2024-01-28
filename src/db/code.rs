use crate::db::{Code, FullCode, Source};
use crate::{PartialCode, Pg};
use rocket_db_pools::sqlx;
use rocket_db_pools::Connection;
use sqlx::PgConnection;

macro_rules! map_row {
    ($row:ident) => {
        FullCode {
            code: Code {
                id: Some($row.code_id),
                code: $row.code.clone(),
                expires_at: $row.expires_at,
                submitter_id: $row.submitter_id,
                creator_id: $row.creator_id,
                lister_id: $row.lister_id,
            },
            creator: Source {
                id: Some($row.creator_id),
                name: $row.creator_name.clone(),
                url: $row.creator_url.clone(),
            },
            submitter: Source {
                id: Some($row.submitter_id),
                name: $row.submitter_name.clone(),
                url: $row.submitter_url.clone(),
            },
            lister: Source {
                id: Some($row.lister_id),
                name: $row.lister_name.clone(),
                url: $row.lister_url.clone(),
            },
        }
    };
}

pub async fn find_codes(
    mut db: Connection<Pg>,
    allow_expired: bool,
) -> Result<Vec<FullCode>, sqlx::Error> {
    sqlx::query!(
        "select
            c.id as code_id,
            c.code,
            c.expires_at,
            creator.id as creator_id,
            creator.name as creator_name,
            creator.url as creator_url,
            submitter.id as submitter_id,
            submitter.name as submitter_name,
            submitter.url as submitter_url,
            lister.id as lister_id,
            lister.name as lister_name,
            lister.url as lister_url
        from codes c
        join sources creator on creator.id = c.creator_id
        join sources submitter on submitter.id = c.submitter_id
        join sources lister on lister.id = c.lister_id
        where (c.expires_at > now() or $1 = true)
        order by c.expires_at desc
        limit 100",
        allow_expired,
    )
    .fetch_all(&mut **db)
    .await?
    .iter()
    .map(|row| Ok(map_row!(row)))
    .collect()
}

pub async fn insert_partial_code(
    conn: &mut PgConnection,
    lister_source_id: i32,
    partial_code: PartialCode,
) -> Result<i32, sqlx::Error> {
    let pdt = partial_code
        .expires_at_to_pdt()
        .unwrap_or_else(|_| crate::util::time_now_add_week());

    // This querty does a few things:
    // create sources if they don't exist, or retrieve the ID if they do
    // insert the code if it doesn't exist
    // return the ID of the code (inserted ID if didn't exist, selected ID if it did) at the end
    // This means it COULD return an ID of a code that you did not add, and that's OK
    sqlx::query!(
        r#"
with
    creator_insert as (
        insert into sources (name, url)
            values ($1, $2)
            on conflict (name, url) do nothing
        returning id
    ),
    submitter_insert as (
        insert into sources (name, url)
            values ($3, $4)
            on conflict (name, url) do nothing
        returning id
    )
insert into codes (code, expires_at, creator_id, submitter_id, lister_id)
    values (
        $7, 
        $5,
        coalesce(
            (select id from creator_insert),
            (select id from sources where name = $1)
        ),
        coalesce(
            (select id from submitter_insert),
            (select id from sources where name = $3)
        ),
        $6
    )
    on conflict (id) do nothing
returning id;
        "#,
        partial_code.creator_name.clone(),
        partial_code.creator_url.clone(),
        partial_code
            .submitter_name
            .unwrap_or_else(|| partial_code.creator_name),
        partial_code
            .submitter_url
            .unwrap_or_else(|| partial_code.creator_url),
        pdt,
        lister_source_id,
        partial_code.code,
    )
    .fetch_one(conn)
    .await
    .map(|record| record.id)
}
