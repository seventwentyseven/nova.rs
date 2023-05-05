use actix_web::{get, guard, web, HttpResponse, Scope};

pub fn osu_handler() -> Scope {
    web::scope("")
        .guard(guard::Header("host", "osu.nikogodomena.me"))
        .service(index)
    //.service( user_avatar)
}

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .body("{'error': 'no uid provided'}")
}
