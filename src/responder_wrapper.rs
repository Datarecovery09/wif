use image::{ImageOutputFormat};
use warp::http;

#[derive(Debug)]
pub struct ResponderWrapper {
    pub content: Vec<u8>,
    pub status_code: Option<http::StatusCode>,
    pub content_type: String
}
impl ResponderWrapper {
    pub fn server_error(message: &str) -> ResponderWrapper {
        ResponderWrapper {
            content: format!("<h1>500 - Internal Server Error</h1><div>{}</div>", message).into_bytes(),
            status_code: Some(http::StatusCode::INTERNAL_SERVER_ERROR),
            content_type: "text/html; charset=UTF-8".to_owned()
        }
    }

    pub fn not_found(message: &str) -> ResponderWrapper {
        ResponderWrapper {
            content: format!("<h1>404 - Not found!</h1><div>{}</div>", message).into_bytes(),
            status_code: Some(http::StatusCode::NOT_FOUND),
            content_type: "text/html; charset=UTF-8".to_owned()
        }
    }

    pub fn not_allowed(message: &str) -> ResponderWrapper {
        ResponderWrapper {
            content: format!("<h1>403 - Forbidden!</h1><div>{}</div>", message).into_bytes(),
            status_code: Some(http::StatusCode::FORBIDDEN),
            content_type: "text/html; charset=UTF-8".to_owned()
        }
    }

    pub fn not_authenticated(message: &str) -> ResponderWrapper {
        ResponderWrapper {
            content: format!("<h1>401 - Not authenticated!</h1><div>{}</div>", message).into_bytes(),
            status_code: Some(http::StatusCode::UNAUTHORIZED),
            content_type: "text/html; charset=UTF-8".to_owned()
        }
    }

    pub fn bad_request(message: &str) -> ResponderWrapper {
        ResponderWrapper {
            content: format!("<h1>400 - Bad Request!</h1><div>{}</div>", message).into_bytes(),
            status_code: Some(http::StatusCode::BAD_REQUEST),
            content_type: "text/html; charset=UTF-8".to_owned()
        }
    }

    pub fn not_implemented(message: &str) -> ResponderWrapper {
        ResponderWrapper {
            content: format!("<h1>400 - Bad Request!</h1><div>{}</div>", message).into_bytes(),
            status_code: Some(http::StatusCode::NOT_IMPLEMENTED),
            content_type: "text/html; charset=UTF-8".to_owned()
        }
    }

    pub fn json(data: String) -> ResponderWrapper {
        ResponderWrapper {
            content: data.into_bytes(),
            status_code: Some(http::StatusCode::OK),
            content_type: "application/json".to_owned()
        }
    }

    pub fn ldjson_text(data: String) -> ResponderWrapper {
        ResponderWrapper {
            content: data.into_bytes(),
            status_code: Some(http::StatusCode::OK),
            content_type: "application/ld+json".to_owned()
        }
    }

    pub fn text(message: &str) -> ResponderWrapper {
        ResponderWrapper {
            content: message.to_owned().into_bytes(),
            status_code: Some(http::StatusCode::OK),
            content_type: "text/html; charset=UTF-8".to_owned()
        }
    }

    pub fn img(data: Vec<u8>, imgtype: &ImageOutputFormat) -> ResponderWrapper {
        let format = match imgtype {
            ImageOutputFormat::Jpeg(_) => "jpeg",
            ImageOutputFormat::Png => "png",
            ImageOutputFormat::Bmp => "bmp",
            ImageOutputFormat::Ico => "ico",
            ImageOutputFormat::Tga => "x-targa",
            _ => "png"
        };

        ResponderWrapper {
            content: data,
            status_code: Some(http::StatusCode::OK),
            content_type: format!("image/{}", format)
        }
    }

    pub fn status_code(&self) -> http::StatusCode {
        match self.status_code {
            Some(v) => v,
            None => http::StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
