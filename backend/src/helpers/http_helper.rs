use axum::{
    body::{Body, Bytes},
    http::{header, HeaderValue, StatusCode},
    response::Response,
};
use rex_game_shared::ApplicationError;

pub struct HttpHelper {}

impl HttpHelper {
    pub fn build_file_respone(
        file_data: Vec<u8>,
        content_type: &str,
    ) -> Result<Response<Body>, ApplicationError> {
        let header_value = match HeaderValue::from_str(content_type) {
            Ok(v) => v,
            Err(_) => return Err(ApplicationError::invalid_input("Invalid content type")),
        };

        match Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, header_value)
            .body(Body::from(Bytes::from(file_data)))
        {
            Ok(response) => Ok(response),
            Err(_) => Err(ApplicationError::invalid_input("Invalid data")),
        }
    }
}
