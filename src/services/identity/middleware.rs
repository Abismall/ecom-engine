use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::cookie::Key;

pub fn session_mw() -> SessionMiddleware<CookieSessionStore> {
    let secret_key = Key::generate();
    SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
        // disable secure cookie for local testing
        .cookie_secure(false)
        .build()
}
