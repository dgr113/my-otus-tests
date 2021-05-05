use diesel::PgConnection;
use diesel::r2d2::{ ConnectionManager, Pool };

use crate::core::errors::AppError;




pub fn get_pg_conn_pool(db_path: &str) -> Result<Pool<ConnectionManager<PgConnection>>, AppError> {
    let db_conn_manager = ConnectionManager::<PgConnection>::new( db_path );
    Pool::builder().build( db_conn_manager ).map_err( AppError::from )
}
