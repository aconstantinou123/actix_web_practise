use actix_web::{ Error, web, HttpResponse, Responder, guard, HttpRequest };
use url::Url;
use std::collections::HashMap;

use super::super::structs::app_state::AppState;
// use super::super::structs::query_params::QueryParams;

fn counter(data: web::Data<AppState>) -> impl Responder {
    let count = data.counter.get() + 1;
    data.counter.set(count);
    format!("Reequest number: {}", count)
}

fn practitioners() -> impl Responder {
    HttpResponse::Ok().body("Hello from practitioners!")
}

fn params(req: HttpRequest) -> Result<String, Error> {
    let name: String = req.match_info().get("friend").unwrap().parse().unwrap();
    let userid: i32 = req.match_info().query("userId").parse().unwrap();
    Ok(format!("Welcome {}, userId {}!", name, userid))
}

fn query_route(req: HttpRequest) -> String {
    let url = format!("http://localhost:8080{}", req.uri());
    let parsed_url = Url::parse(&url).unwrap();
    let hash_query: HashMap<_, _> = parsed_url.query_pairs().into_owned().collect();
    format!("{:?}", hash_query)
}

pub fn practitioner_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/practitioners")
            .guard(guard::Header("Host", "localhost:8080"))
            .route("/", web::get().to(practitioners)) 
            .route("/counter", web::get().to(counter))
            .route("/params/{userId}/{friend}", web::get().to(params))
            .route("/query", web::get().to(query_route))
    );
}
