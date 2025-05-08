// #[derive(Serialize, Deserialize)]
// pub struct ErrorRes {
//     status: String,
//     data: &'static str,
// }

use core::fmt;

#[derive(Debug)]
pub enum Error {
    Grid(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Grid(msg) => write!(f, "Grid: {}", msg),
        }
    }
}

// impl ResponseError for Error {
//     fn error_response(&self) -> HttpResponse {
//         match self {
//             Error::Conflict(msg) => HttpResponse::Conflict().json(ErrorRes {
//                 status: msg.to_string(),
//                 data: "",
//             }),
//             Error::Unauthorized(msg) => HttpResponse::Unauthorized().json(ErrorRes {
//                 status: msg.to_string(),
//                 data: "",
//             }),
//             Error::BadRequest(msg) => HttpResponse::BadRequest().json(ErrorRes {
//                 status: msg.to_string(),
//                 data: "",
//             }),
//             Error::Internal(msg) => HttpResponse::InternalServerError().json(ErrorRes {
//                 status: format!("server skillissue: {}", msg),
//                 data: "",
//             }),
//             Error::UniqueNameViolation(msg) => HttpResponse::Conflict().json(ErrorRes {
//                 status: msg.to_string(),
//                 data: "",
//             }),
//             Error::NotFound(msg) => HttpResponse::NotFound().json(ErrorRes {
//                 status: msg.to_string(),
//                 data: "",
//             }),
//         }
//     }
// }
