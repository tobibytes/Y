use crate::secrets::SECRET_MANAGER;
use once_cell::sync::Lazy;
use sqlx::{
    FromRow, Pool, Postgres,
    postgres::{PgArguments, PgPoolOptions, PgRow},
    query::QueryAs,
};

static DATABASE: Lazy<Pool<Postgres>> = Lazy::new(|| {
    let db_uri = SECRET_MANAGER.get("DB_URI");
    PgPoolOptions::new()
        .max_connections(5)
        .connect_lazy(&db_uri)
        .expect("invalid DATABASE_URL")
});
pub async fn fetch_one_with<T>(
    sql: &'static str,
    bind: impl FnOnce(
        QueryAs<'static, Postgres, T, PgArguments>,
    ) -> QueryAs<'static, Postgres, T, PgArguments>,
) -> Result<T, sqlx::Error>
where
    T: for<'r> FromRow<'r, PgRow> + Send + Unpin,
{
    let q = sqlx::query_as::<_, T>(sql);
    let q = bind(q);
    q.fetch_one(&*DATABASE).await
}

// pub async fn fetch_all_with<T>(
//     sql: &'static str,
//     bind: impl FnOnce(QueryAs<'static, Postgres, T, PgArguments>) -> QueryAs<'static, Postgres, T, PgArguments>,
// ) -> Result<Vec<T>, sqlx::Error>
// where
//     T: for<'r> FromRow<'r, PgRow> + Send + Unpin,
// {
//     let q = sqlx::query_as::<_, T>(sql);
//     let q = bind(q);
//     q.fetch_all(&*DATABASE).await
// }
// pub fn pool() -> &'static Pool<Postgres> {
//     &*DATABASE
// }
