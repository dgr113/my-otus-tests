use actix_web::HttpRequest;
use serde::{ Serialize, Deserialize };

use crate::core::errors::AppError;
use crate::db::ops::update::OrderUpdate;
use crate::db::ops::create::{ OrderProductCreate, OrderCreate };




pub struct ReqHeader {
    pub(crate) account_id: i32
}

impl ReqHeader {
    pub fn parse(request: &HttpRequest) -> Result<Self, AppError> {
        let account_id = request.headers().get("X-Account-Id")
            .ok_or( AppError::ParseHeaderError( "'X-Account-Id' request header not found".to_string() ) ) ?
            .to_str().expect( "Unexpected getting header error as string" )
            .parse::<i32>()
            .map_err( |err| AppError::ParseHeaderError( "'X-Account-Id' error parse into number".to_string() ) ) ?;

        Ok( Self {
            account_id
        })
    }
}




#[derive(Clone, Serialize, Deserialize)]
pub struct CreateOrderData {
    pub idempotency_key: String,
    pub order: OrderCreate,
    pub order_products: Vec<OrderProductCreate>
}



#[derive(Clone, Serialize, Deserialize)]
pub struct UpdateOrderData {
    pub order: OrderUpdate
}
