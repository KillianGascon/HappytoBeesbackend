use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool as DieselPool, PoolError};
use log::error;

pub type DbConnection = PgConnection;
pub type PooledConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection_pool(database_url: &str) -> Result<Pool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}