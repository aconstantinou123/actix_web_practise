use actix_web::{ web, HttpResponse, Responder, guard };
use super::super::structs::app_state::AppState;

fn counter(data: web::Data<AppState>) -> impl Responder {
    let count = data.counter.get() + 1;
    data.counter.set(count);
    format!("Reequest number: {}", count)
}

fn practitioners() -> impl Responder {
    HttpResponse::Ok().body("Hello from practitioners!")
}

pub fn practitioner_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/practitioners")
            .guard(guard::Header("Host", "localhost:8080"))
            .route("/", web::get().to(practitioners)) 
            .route("/counter", web::get().to(counter))
    );
}
