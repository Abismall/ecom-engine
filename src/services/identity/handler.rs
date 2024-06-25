use actix_identity::Identity;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder};

pub async fn index(user: Option<Identity>) -> impl Responder {
    if let Some(user) = user {
        format!("Welcome! {}", user.id().unwrap())
    } else {
        "Welcome Anonymous!".to_owned()
    }
}

pub async fn login(request: HttpRequest) -> impl Responder {
    // Some kind of authentication should happen here
    // e.g. password-based, biometric, etc.
    // [...]

    // attach a verified user identity to the active session
    Identity::login(&request.extensions(), "User1".into()).unwrap();

    HttpResponse::Ok()
}

pub async fn logout(user: Identity) -> impl Responder {
    user.logout();
    HttpResponse::Ok()
}