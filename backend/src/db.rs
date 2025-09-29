use once_cell::sync::Lazy;
use sqlx::{FromRow, Pool, Postgres, postgres::{ PgPoolOptions, PgRow}};
use crate::secrets::SECRET_MANAGER;

static DATABASE: Lazy<Pool<Postgres>> = Lazy::new(|| {
    let db_uri = SECRET_MANAGER.get("DB_URI");
    PgPoolOptions::new()
        .max_connections(5)
        .connect_lazy(&db_uri)  
        .expect("invalid DATABASE_URL")
});

pub async fn run_query<T>(query: &str) -> Result<Vec<T>, sqlx::Error> 
where
T: for<'r> FromRow<'r, PgRow> + Send + Unpin{

    let rows = sqlx::query_as::<_, T>(query)
        .fetch_all(&*DATABASE)
        .await?;
        // .expect("Failed to execute query");
    Ok(rows)
}

pub fn pool() -> &'static Pool<Postgres> {
    &*DATABASE
}
