use actix_web::{web, App};
use ecom_engine::logger::logger::DETAILED_FORMAT;
use ecom_engine::{
    api::rest::{local_dev_cors, local_dev_headers, pg_r2d2_connection_pool},
    services::brand::service::configure as brand_service,
    services::cart::service::configure as cart_service,
    services::discount::service::configure as discount_service,
    services::order::service::configure as order_service,
    services::product::service::configure as product_service,
};
async fn index() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().json("Hello, RestApiService!")
}

const DB_CON_RETRY_ATTEMPTS: u8 = 3;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv::dotenv().ok(); // Load environment variables from .env file
    
    env_logger::init();
    let cfg = ecom_engine::cfg::file::Config::from_file("config.json");
    let env = ecom_engine::cfg::file::Env::from_config(&cfg.clone());
    let host_clone = env.api_host.clone();
    let port_clone = env.api_port.clone();
    let mut attempts = 0;
    let pool = loop {
        match pg_r2d2_connection_pool(&env.db_url) {
            Ok(pool) => break pool,
            Err(err) => {
                attempts += 1;
                if attempts >= DB_CON_RETRY_ATTEMPTS {
                    log::error!(
                        "An error occurred while creating the database connection pool: {}",
                        err
                    );
                    std::process::exit(1);
                } else {
                    log::warn!("Attempt {} failed, retrying...", attempts);
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                }
            }
        }
    };

    actix_web::HttpServer::new(move || {
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
            .route("/", web::get().to(index))
            .wrap(cors)
            .wrap(headers)
            .wrap(actix_web::middleware::Logger::new(DETAILED_FORMAT))
            .configure(brand_service)
            .configure(product_service)
            .configure(cart_service)
            .configure(discount_service)
            .configure(order_service)
            .app_data(web::Data::new(pool.clone()))
    })
    .bind((host_clone, port_clone.parse::<u16>().unwrap()))?
    .run()
    .await
}
