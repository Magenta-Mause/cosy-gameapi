use actix_web::{
    http::{self, StatusCode},
    Responder,
};
use serde::Serialize;

#[derive(Serialize, Clone, Default)]
pub struct Response<T: Serialize> {
    success: bool,
    timestamp: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,

    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,

    #[serde(skip_serializing)]
    code: Option<http::StatusCode>,
}

impl<T: Serialize> Response<T> {
    pub fn success(data: T) -> Self {
        Response {
            success: true,
            data: Some(data),
            message: None,
            code: None,
            timestamp: 0,
        }
    }

    pub fn error(message: String, code: http::StatusCode) -> Self {
        Self {
            success: false,
            data: None,
            message: Some(message),
            code: Some(code),
            timestamp: 0,
        }
    }
}

impl<T: Serialize> Responder for Response<T> {
    type Body = actix_web::body::BoxBody;

    fn respond_to(mut self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        self.success = self.code.unwrap_or(StatusCode::OK).is_success();
        self.timestamp = chrono::Utc::now().timestamp_millis() as u64;
        actix_web::HttpResponse::build(self.code.unwrap_or(StatusCode::OK)).json(self)
    }
}
