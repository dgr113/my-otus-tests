use actix_web::web::Json;
use prometheus::{ Encoder, TextEncoder, gather };

use crate::core::errors::AppError;
use crate::core::responses::ResponseHealth;
use crate::prometheus::metrics::REGISTRY;




/** Service health check */
pub async fn service_status() -> Result<Json<ResponseHealth>, AppError>  {
    let resp = ResponseHealth { status: "OK".to_string() };
    Ok( Json( resp ) )
}



/** Prometheus metric handler */
pub async fn metrics_handler() -> Result<String, AppError> {
    let encoder = TextEncoder::new();

    // Getting metrics from app registry
    let mut buffer = Vec::new();
    if let Err(e) = encoder.encode(&REGISTRY.gather(), &mut buffer) {
        eprintln!("could not encode custom metrics: {}", e.to_string());
    };
    let mut res = match String::from_utf8(buffer.clone()) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("custom metrics could not be from_utf8'd: {}", e.to_string());
            String::default()
        }
    };
    buffer.clear();

    // Getting metrics from system registry
    let mut buffer = Vec::new();
    if let Err(e) = encoder.encode(&gather(), &mut buffer) {
        eprintln!("could not encode prometheus metrics: {}", e.to_string());
    };
    let res_custom = match String::from_utf8(buffer.clone()) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("prometheus metrics could not be from_utf8'd: {}", e);
            String::default()
        }
    };
    buffer.clear();

    res.push_str(&res_custom);
    Ok( res )
}
