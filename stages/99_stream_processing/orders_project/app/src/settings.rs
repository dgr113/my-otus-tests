use serde::Deserialize;

use config::{ ConfigError, Config, Environment };




#[derive(Debug, Clone, Deserialize)]
pub struct KfkConfig {
    pub addr: String,
    pub producer_topic: String,
    pub consumer_topic: String,
    pub group: String
}
impl KfkConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let mut conf = Config::new();
        conf.merge( Environment::with_prefix( "KAFKA" ) ) ?;
        conf.try_into::<Self>()
    }
}



#[derive(Debug, Clone, Deserialize)]
pub struct DbConfig {
    pub driver: String,
    pub host: String,
    pub port: String,
    pub name: String,
    pub user: String,
    pub password: String
}
impl DbConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let mut conf = Config::new();
        conf.merge( Environment::with_prefix( "DB" ) ) ?;
        conf.try_into::<Self>()
    }
}
