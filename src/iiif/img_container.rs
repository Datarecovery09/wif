extern crate image;
use image::{DynamicImage, ImageOutputFormat};
use std::path::Path;

use super::parsing_error::ErrorReturnType;


pub struct ImgContainer {
    pub img: DynamicImage
}

impl ImgContainer {
    pub fn from_file(path: &str) -> Result<Self, ErrorReturnType> {
        if !Path::new(path).exists() {
            return Err(ErrorReturnType::NotFound("File not found".to_owned()))
        }

        let dyn_img = match image::open(path) {
            Ok(v) => v,
            Err(e) => return Err(ErrorReturnType::InternalError(format!("{:?}", e)))
        };

        let res = Self {
            img: dyn_img
        };

        Ok(res)
    }

    pub fn body(&self, format: ImageOutputFormat) -> Result<Vec<u8>, ErrorReturnType> {
        let mut buf: Vec<u8> = vec![];
        match self.img.write_to(&mut buf, format) {
            Ok(_) => (),
            Err(e) => return Err(ErrorReturnType::InternalError(format!("Could not write DynamicImage to buffer --- {:?}", e)))
        }
        Ok(buf)
    }
}


