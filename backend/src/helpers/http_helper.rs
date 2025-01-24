use axum::{
    body::{Body, Bytes},
    http::{header, HeaderValue, StatusCode},
    response::Response,
};

pub struct HttpHelper {}

impl HttpHelper {
    pub fn build_file_respone(file_data: Vec<u8>) -> Response<Body> {
        Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, HeaderValue::from_static("image/jpeg"))
            .body(Body::from(Bytes::from(file_data)))
            .unwrap()
    }
}
