use diesel::PgConnection;
use diesel::r2d2::{ Pool, ConnectionManager, PooledConnection };



pub type ConnManager = ConnectionManager<PgConnection>;
pub type ConnPool = Pool<ConnManager>;
pub type PooledConn = PooledConnection<ConnManager>;
