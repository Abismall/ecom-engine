use std::sync::Arc;

use actix_web::{web, App, HttpServer};
use ecom_engine::{
    api::rest::local_dev_headers,
    cfg,
    logger::logger::DETAILED_FORMAT,
    stream::{create_add_update_route, StreamChannel, UpdateProcessor},
    CONFIG_FILE_PATH,
};
use tokio::sync::{mpsc, Notify};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let env_config: cfg::Env = cfg::Config::from_file(CONFIG_FILE_PATH).into();

    let redis_url = Arc::new(env_config.redis_url);
    let notify = Arc::new(Notify::new());

    let (tx, rx) = mpsc::channel(100);

    let processor = Arc::new(UpdateProcessor::new(redis_url.to_string()).await);

    let processor_clone = Arc::clone(&processor);

    let product_update_channel_key: String = StreamChannel::ProductUpdates.into();

    tokio::spawn(async move {
        processor_clone
            .process_updates(product_update_channel_key, rx)
            .await;
    });

    HttpServer::new(move || {
        App::new()
            .wrap(local_dev_headers())
            .wrap(actix_web::middleware::Logger::new(DETAILED_FORMAT))
            .wrap(
                actix_cors::Cors::default()
                    .send_wildcard()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
                    .allowed_headers(vec!["Content-Type", "Authorization"])
                    .max_age(3600),
            )
            .app_data(web::Data::new(notify.clone()))
            .app_data(web::Data::new(redis_url.clone()))
            .app_data(web::Data::new(tx.clone()))
            .app_data(web::Data::new(processor.clone()))
            .configure(create_add_update_route)
    })
    .bind("0.0.0.0:3030")?
    .run()
    .await
}
