use actix_web::{web, App};
use ecom_engine::api::rest::{resolve_connection_pool, DEFAULT_PORT};
use ecom_engine::logger::logger::DETAILED_FORMAT;
use ecom_engine::{
    api::rest::{local_dev_cors, local_dev_headers},
    services::brand::service::configure as brand,
    services::cart::service::configure as cart,
    services::discount::service::configure as discount,
    services::order::service::configure as order,
    services::product::service::configure as product,
    services::stock::service::configure as stock,
};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv::dotenv().ok(); // Load environment variables from .env file

    env_logger::init();
    let cfg = ecom_engine::cfg::Config::from_file("config.json");
    let env = ecom_engine::cfg::Env::from_config(&cfg.clone());
    let host_clone = env.api_host.clone();
    let port_clone = env.api_port.clone();
    let pool = resolve_connection_pool(&env.db_url).await;

    actix_web::HttpServer::new(move || {
        let logger = actix_web::middleware::Logger::new(DETAILED_FORMAT);
        let pool_clone = pool.clone();
        let pool_app_data = web::Data::new(pool_clone);
        let cors = match env.api_host.as_str() {
            "localhost" | "0.0.0.0" | "127.0.0.1" => local_dev_cors(),
            _ => actix_cors::Cors::default()
                .allowed_origin(&env.api_host)
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
                .allowed_headers(vec!["Content-Type", "Authorization"])
                .max_age(3600),
        };
        let headers = match env.api_host.as_str() {
            "localhost" | "0.0.0.0" | "127.0.0.1" => local_dev_headers(),
            _ => local_dev_headers(),
        };

        App::new()
            .wrap(cors)
            .wrap(headers)
            .wrap(logger)
            .configure(brand)
            .configure(product)
            .configure(cart)
            .configure(discount)
            .configure(order)
            .configure(stock)
            .app_data(pool_app_data)
    })
    .bind((host_clone, port_clone.parse::<u16>().unwrap_or(DEFAULT_PORT)))?
    .run()
    .await
}
