mod structs;
mod routes;

use actix_web::{ App, HttpServer };
use listenfd::ListenFd;
use openssl::ssl::{ SslAcceptor, SslFiletype, SslMethod };

use structs::app_state::AppState;
use std::cell::Cell;
use routes::patient_routes::patient_routes;
use routes::practitioner_routes::practitioner_routes;
use std::path::Path;

fn main() {
    let key = Path::new("/Users/alexconstantinou/projects/rust/actix_web/hello-actix/src/key.pem");
    let cert = Path::new("/Users/alexconstantinou/projects/rust/actix_web/hello-actix/src/cert.pem");

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .data(AppState {
                counter: Cell::new(0)
            })
            .configure(patient_routes)    
            .configure(practitioner_routes)
    });
    //Create temp ssl cert (self-signed)
    let mut builder =
        SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(&key, SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file(&cert).unwrap();

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server
            .bind("127.0.0.1:8080").unwrap()
    };

    server.run().unwrap();
}