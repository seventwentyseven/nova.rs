use crate::config::CONFIG;
use actix_web::{get, guard, web, HttpResponse, Scope};
use std::path::PathBuf;

pub fn ava_handler() -> Scope {
    web::scope("")
        .guard(guard::Header("host", "a.nikogodomena.me"))
        .service(index)
        .service(user_avatar)
}

const ALLOWED_EXTENSIONS: [&str; 4] = ["png", "jpg", "jpeg", "gif"];

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .body("{'error': 'no uid provided'}")
}

#[get("/{userid}")]
async fn user_avatar(userid: web::Path<u32>) -> HttpResponse {
    let mut path = PathBuf::from(&CONFIG.General.path);
    path.push(".data");
    path.push("avatars");

    for ext in &ALLOWED_EXTENSIONS {
        let mut file = path.clone();
        file.push(format!("{}.{}", userid, ext));
        if file.exists() {
            return HttpResponse::Ok().body(std::fs::read(file).unwrap());
        }
    }
    HttpResponse::NotFound().body("404")
}
