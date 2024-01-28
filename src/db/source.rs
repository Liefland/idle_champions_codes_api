use crate::db::Source;
use crate::Pg;
use rocket_db_pools::sqlx;
use rocket_db_pools::Connection;

pub async fn insert_source(
    mut db: Connection<Pg>,
    name: String,
    url: String,
) -> Result<u64, sqlx::Error> {
    let record = sqlx::query!(
        "insert into sources (name, url) values ($1, $2) returning id",
        name,
        url
    )
    .fetch_one(&mut **db)
    .await?;

    Ok(record.id.try_into().unwrap())
}

pub async fn find_source(mut db: Connection<Pg>, id: i32) -> Option<Source> {
    sqlx::query!("select id, name, url from sources where id = $1", id)
        .fetch_one(&mut **db)
        .await
        .map(|record| {
            Some(Source {
                id: Some(record.id),
                name: record.name,
                url: record.url,
            })
        })
        .map_err(|_| None::<Source>)
        .unwrap()
}

pub async fn find_source_by_name(mut db: Connection<Pg>, name: String) -> Option<Source> {
    sqlx::query!("select id, name, url from sources where name = $1", name)
        .fetch_one(&mut **db)
        .await
        .map(|record| {
            Some(Source {
                id: Some(record.id),
                name: record.name,
                url: record.url,
            })
        })
        .map_err(|_| None::<Source>)
        .unwrap()
}
