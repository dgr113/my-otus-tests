use serde::{ Serialize, Deserialize };
use std::fmt::{ Display, Formatter, Result as FmtResult };




#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum KfkEventType {
    ORDER_CREATED,
    ORDER_PAID_SUCCESS,
    ORDER_PAID_FAILED,
    ORDER_NOTIFY_SUCCESS,
    UNKNOWN
}

impl From<&str> for KfkEventType {
    fn from(s: &str) -> Self {
        match s {
            "ORDER_CREATED" => KfkEventType::ORDER_CREATED,
            "ORDER_PAID_SUCCESS" => KfkEventType::ORDER_PAID_SUCCESS,
            "ORDER_PAID_FAILED" => KfkEventType::ORDER_PAID_FAILED,
            "ORDER_NOTIFY_SUCCESS" => KfkEventType::ORDER_NOTIFY_SUCCESS,
            _ => KfkEventType::UNKNOWN
        }
    }
}

impl Display for KfkEventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let msg = match self {
            Self::ORDER_CREATED => String::from( "ORDER_CREATED" ),
            Self::ORDER_PAID_SUCCESS => String::from( "ORDER_PAID_SUCCESS" ),
            Self::ORDER_PAID_FAILED => String::from( "ORDER_PAID_FAILED" ),
            Self::ORDER_NOTIFY_SUCCESS => String::from( "ORDER_NOTIFY_SUCCESS" ),
            Self::UNKNOWN => String::from( "UNKNOWN" )
        };
        write!(f, "{}", msg)
    }
}
