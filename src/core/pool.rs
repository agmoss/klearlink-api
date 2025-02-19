use rocket_sync_db_pools::{database, diesel};

#[database("klearlink")]
pub struct Db(diesel::PgConnection);
