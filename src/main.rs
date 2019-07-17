mod structs;
mod routes;

use actix_web::{ App, HttpServer };
use listenfd::ListenFd;
use structs::app_state::AppState;
use std::cell::Cell;
use routes::patient_routes::patient_routes;
use routes::practitioner_routes::practitioner_routes;

fn main() {
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .data(AppState {
                counter: Cell::new(0)
            })
            .configure(patient_routes)    
            .configure(practitioner_routes)
    });
    
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind("127.0.0.1:8080").unwrap()
    };

    server.run().unwrap();
}