use actix_cors::Cors;
use std::time::Instant;
use r2d2::Pool as R2D2Pool;
use actix_web::web::JsonConfig;
use futures::future::FutureExt;
use actix_web::error::InternalError;
use actix_service::{ ServiceFactory, Service };
use actix_web::{
    web, App, Error, HttpResponse, body::Body,
    dev::{ ServiceResponse, ServiceRequest }
};

use crate::views;
use crate::app_types::ConnManager;
use crate::prometheus::metrics::{ INCOMING_REQUESTS, RESPONSE_TIME_COLLECTOR, RESPONSE_CODE_COLLECTOR };




#[derive(Clone)]
pub struct AppData {
    pub db_pool: R2D2Pool<ConnManager>
}




pub struct AppFactory;

impl AppFactory {
    /** Build app with views and data **/
    pub fn create_app()
        -> App<
            impl ServiceFactory<Config = (), Request = ServiceRequest, Response = ServiceResponse, Error = Error, InitError = ()>,
            Body
        >
    {
        let cors_conf = Cors::default()
            .allow_any_method()
            .allow_any_header()
            .allow_any_origin()
            .supports_credentials();

        let json_conf = JsonConfig::default()
            .limit( 2048 )
            .error_handler(|err, _| {
                let resp = HttpResponse::BadRequest().json( hashmap! { "error" => err.to_string() } );
                InternalError::from_response(err, resp).into()
            });

        App::new()
            .wrap_fn(|req, srv| {
                let req_method = req.method().to_string();
                let req_path = req.path().to_string();
                let start = Instant::now();

                INCOMING_REQUESTS.with_label_values( &[&*req_method, &*req_path] ).inc();  // Update `incoming requests` metric

                srv.call( req ).map(move |resp| {
                    let resp_code = resp.as_ref().map( |r| r.status().to_string() ).unwrap_or( String::from("500") );
                    RESPONSE_TIME_COLLECTOR.with_label_values( &[&*req_method, &*req_path] ).observe( start.elapsed().as_secs() as f64 );  // Update `response time` metric
                    RESPONSE_CODE_COLLECTOR.with_label_values( &[&*req_method, &*req_path, &resp_code] ).inc();  // Update `response code` metric
                    resp
                })
            })

            .wrap( cors_conf )
            .app_data( json_conf )

            .route("/status/", web::get().to( views::health::service_status ))
            .route("/metrics/", web::get().to( views::health::metrics_handler ))

            .service(
                web::resource("/a/")
                    .route( web::get().to( views::users::get_by_filter ) )
                    .route( web::post().to( views::users::create ) )
            )
            .service(
                web::resource("/a/{id}/")
                    .route( web::get().to( views::users::get_by_id ) )
                    .route( web::put().to( views::users::update_by_id ) )
                    .route( web::delete().to( views::users::delete_by_id ) )
            )
    }
}
