use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{Responder, Response};
use rocket::serde::json::Json;
use serde::Serialize;
use rocket::http::ContentType;

#[derive(Serialize)]
pub struct StatusResponse<T> {
    pub status: u16,
    pub data: T,
}

impl<'request, 'response: 'request, T: Serialize> Responder<'request, 'response> for StatusResponse<T> {
    fn respond_to(self, request: &'request Request<'_>) -> rocket::response::Result<'response> {
        let json_response = Json(&self.data);

        Response::build_from(json_response.respond_to(request)?)
            .status(Status::from_code(self.status).unwrap())
            .header(ContentType::JSON)
            .ok()
    }
}