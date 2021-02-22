extern crate image;
use image::{ImageOutputFormat};

use std::str::FromStr;
use crate::config;

use super::parsing_error::{ErrorReturnType};
use super::img_container::ImgContainer;

#[derive(Debug)]
pub enum EPicQuality {
    Color(ImageOutputFormat),
    Gray(ImageOutputFormat),
    Bitonal(ImageOutputFormat),
    Default(ImageOutputFormat)
}

pub fn mutate_image_quality(quality: &mut EPicQuality, img: &mut ImgContainer) -> Result<ImageOutputFormat, ErrorReturnType> {
    match quality {
        EPicQuality::Color(f) | EPicQuality::Default(f) => {
            Ok(f.clone())
        },
        EPicQuality::Gray(f) => {
            img.img = img.img.grayscale();
            Ok(f.clone())
        },
        EPicQuality::Bitonal(f) => {
            img.img = img.img.grayscale();
            img.img = img.img.adjust_contrast(1000.0);
            Ok(f.clone())
        }
    }
}

impl FromStr for EPicQuality {
    type Err = ErrorReturnType;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() > 1 {
            let format = match &parts[1].to_lowercase() as &str {
                "jpg" | "jpeg" => ImageOutputFormat::Jpeg(config::jpg_quality()),
                "png" => ImageOutputFormat::Png,
                "bmp" => ImageOutputFormat::Bmp,
                "ico" => ImageOutputFormat::Ico,
                "tga" => ImageOutputFormat::Tga,
                _ => return Err(ErrorReturnType::BadRequest("Cannot parse format".to_owned()))
            };

            match parts[0] {
                "color" => return Ok(EPicQuality::Color(format)),
                "gray" => return Ok(EPicQuality::Gray(format)),
                "bitonal" => return Ok(EPicQuality::Bitonal(format)),
                "default" => return Ok(EPicQuality::Default(format)),
                _ => return Err(ErrorReturnType::BadRequest("Cannot parse quality".to_owned()))
            }
        } else {
            return Err(ErrorReturnType::BadRequest("Cannot parse quality and/or format".to_owned()))
        }
    }
}
