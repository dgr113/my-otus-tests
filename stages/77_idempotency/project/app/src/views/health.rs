use actix_web::web::Json;

use crate::core::errors::AppError;
use crate::core::responses::ResponseHealth;




/** Service health check */
pub async fn service_status() -> Result<Json<ResponseHealth>, AppError>  {
    let resp = ResponseHealth { status: "OK".to_string() };
    Ok( Json( resp ) )
}
