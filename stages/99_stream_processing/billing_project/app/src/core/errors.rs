use std::env::VarError;
use std::io::{ Error as IOError };
use std::fmt::{ Formatter, Display, Result as FmtResult };

use config::ConfigError;
use r2d2::Error as DbPoolError;
use kafka::Error as KafkaError;
use actix_web::http::StatusCode;
use diesel::result::Error as DbError;
use serde::{ Serialize, Deserialize };
use log4rs::config::InitError as LogError;
use serde_json::Error as JsonSerializeError;
use actix_web::{ ResponseError, HttpResponse };




#[derive(Debug, Serialize, Deserialize)]
pub enum AppError {
    EnvError( String ),
    IOError( String ),
    LogError( String ),
    DbError( String ),
    KafkaError( String ),
    SerializeError( String ),
    NotEnoughMoney
}

impl From<VarError> for AppError {
    fn from( err: VarError ) -> AppError {
        AppError::EnvError( err.to_string() )
    }
}
impl From<DbPoolError> for AppError {
    fn from( err: DbPoolError ) -> AppError {
        AppError::DbError( err.to_string() )
    }
}
impl From<JsonSerializeError> for AppError {
    fn from( err: JsonSerializeError ) -> AppError {
        AppError::SerializeError( err.to_string() )
    }
}
impl From<KafkaError> for AppError {
    fn from( err: KafkaError ) -> AppError {
        AppError::KafkaError( err.to_string() )
    }
}
impl From<ConfigError> for AppError {
    fn from( err: ConfigError ) -> AppError {
        AppError::EnvError( err.to_string() )
    }
}
impl From<DbError> for AppError {
    fn from( err: DbError ) -> AppError {
        AppError::DbError( format!("Database Error:{}", err.to_string()) )
    }
}
impl From<IOError> for AppError {
    fn from( err: IOError ) -> AppError {
        AppError::IOError( err.to_string() )
    }
}
impl From<LogError> for AppError {
    fn from( err: LogError ) -> AppError {
        AppError::LogError( err.to_string() )
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let msg = match self {
            Self::IOError( t ) => t.to_string(),
            Self::EnvError( t ) => t.to_string(),
            Self::LogError( t ) => t.to_string(),
            Self::DbError( t ) => t.to_string(),
            Self::KafkaError( t ) => t.to_string(),
            Self::SerializeError( t ) => t.to_string(),
            Self::NotEnoughMoney => String::from( "Not enough money!" )
        };
        write!(f, "{}", msg)
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::IOError( _ ) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::EnvError( _ ) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::LogError( _ ) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::DbError( _ ) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::KafkaError( _ ) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::SerializeError( _ ) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotEnoughMoney => StatusCode::INTERNAL_SERVER_ERROR
        }
    }
    fn error_response(&self) -> HttpResponse {
        // error!(target: "errors", "{}", &self.to_string());
        HttpResponse::build( self.status_code() ).json( hashmap! { "error" => self.to_string() } )
    }
}
