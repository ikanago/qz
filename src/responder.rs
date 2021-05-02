// use crate::{
//     body::Body, header::HeaderName, mime, response::Response, status::StatusCode, Version,
// };
// use std::collections::HashMap;

// /// Builder of `Response`.
// pub trait Responder: Sized {
//     fn respond_to(self) -> Response;
// }

// impl Responder for () {
//     fn respond_to(self) -> Response {
//         Response::default()
//     }
// }

// impl Responder for Response {
//     fn respond_to(self) -> Response {
//         self
//     }
// }

// impl Responder for StatusCode {
//     fn respond_to(self) -> Response {
//         Response {
//             status_code: self,
//             version: Version::default(),
//             headers: HashMap::default(),
//             body: Body::default(),
//         }
//     }
// }

// impl Responder for &'static str {
//     fn respond_to(self) -> Response {
//         Response {
//             status_code: StatusCode::Ok,
//             version: Version::default(),
//             headers: vec![
//                 (
//                     HeaderName::ContentLength,
//                     self.len().to_string().as_bytes().to_vec(),
//                 ),
//                 (HeaderName::ContentType, mime::TEXT_PLAIN.to_vec()),
//             ]
//             .into_iter()
//             .collect(),
//             body: Body::from(self),
//         }
//     }
// }

// impl Responder for Vec<u8> {
//     fn respond_to(self) -> Response {
//         Response {
//             status_code: StatusCode::Ok,
//             version: Version::default(),
//             headers: vec![(
//                 HeaderName::ContentLength,
//                 self.len().to_string().as_bytes().to_vec(),
//             )]
//             .into_iter()
//             .collect(),
//             body: Body::from(self),
//         }
//     }
// }
