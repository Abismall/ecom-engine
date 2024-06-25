use actix_web::web;

use super::handler::{index, login, logout};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/identity")
            .route("", web::post().to(index))
            .route("logout", web::post().to(logout))
            .route("/login", web::post().to(login)),
    );
}
