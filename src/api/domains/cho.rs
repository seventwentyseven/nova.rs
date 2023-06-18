use actix_web::{get, guard, web, HttpResponse, Scope};

pub fn cho_handler() -> Scope {
    web::scope("")
        .guard(guard::Header("host", "c.nikogodomena.me"))
        .service(index)
    //.service( user_avatar)
}

// TODO: Basic handler for login
#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .body("{'error': 'no uid provided'}")
}
