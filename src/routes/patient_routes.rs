use actix_web::{ web, HttpResponse, Responder };

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello from patients!")
}

fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}

pub fn patient_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/patients")
            .route("/", web::get().to(index))
            .route("/again", web::get().to(index2))
        );
}