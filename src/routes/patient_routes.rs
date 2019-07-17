use actix_web::{Error, web, HttpResponse, Responder };
use futures::future::{ok, Future};
use super::super::structs::custom_object::CustomObject;

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello from patients!")
}

fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}

fn return_object() -> impl Responder {
    CustomObject { name: "Alex" }
}

fn future1() -> Box<Future<Item = HttpResponse, Error = Error>> {
    Box::new(ok::<_, Error> (
        HttpResponse::Ok().content_type("text/html").body("Hello"),
    ))
}

fn future2() -> Box<Future<Item = &'static str, Error = Error>> {
    Box::new(ok::<_, Error>("Welcome!"))
}

pub fn patient_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/patients")
            .route("/", web::get().to(index))
            .route("/again", web::get().to(index2))
            .route("/object", web::get().to(return_object))
            .route("future-one", web::get().to(future1))
            .route("future-two", web::get().to(future2))
        );
}