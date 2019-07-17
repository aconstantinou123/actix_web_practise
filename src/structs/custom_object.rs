use actix_web::{ Error, HttpRequest, HttpResponse, Responder };
use serde::Serialize;

#[derive(Serialize)]
pub struct CustomObject {
    pub name: &'static str,
}

impl Responder for CustomObject {
    type Error = Error;
    type Future = Result<HttpResponse, Error>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self)?;
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
    }
}

