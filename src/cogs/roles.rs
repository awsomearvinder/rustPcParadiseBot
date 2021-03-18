// temporary.
#[allow(dead_code)]
struct RoleManager<T: sqlx::Connection + 'static> {
    database: crate::db::Db<T, Result<crate::db::QueryOutput, anyhow::Error>>,
}
