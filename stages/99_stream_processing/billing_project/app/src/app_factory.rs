use actix_cors::Cors;
use r2d2::Pool as R2D2Pool;
use actix_web::web::JsonConfig;
use actix_service::ServiceFactory;
use actix_web::error::InternalError;
use actix_web::{
    web, App, Error, HttpResponse, body::Body,
    dev::{ ServiceResponse, ServiceRequest }
};

use crate::views;
use crate::app_types::ConnManager;




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
            .wrap( cors_conf )
            .app_data( json_conf )

            .route("/status/", web::get().to( views::health::service_status ))

            .service(
                web::resource("/a/")
                    .route( web::post().to( views::accounts::create ) )
            )
            .service(
                web::resource("/a/{id}/")
                    .route( web::get().to( views::accounts::get_by_id ) )
                    .route( web::put().to( views::accounts::update_by_id ) )
                    .route( web::delete().to( views::accounts::delete_by_id ) )
            )
    }
}
