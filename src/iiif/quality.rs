extern crate image;
use image::{DynamicImage, ImageOutputFormat};

use std::str::FromStr;
use crate::config;

use crate::wif_error::WifError;

#[derive(Debug)]
pub enum EPicQuality {
    Color(ImageOutputFormat),
    Gray(ImageOutputFormat),
    Bitonal(ImageOutputFormat),
    Default(ImageOutputFormat)
}

pub fn mutate_image_quality(quality: &mut EPicQuality, img: &mut DynamicImage) -> Result<(Vec<u8>, ImageOutputFormat), WifError> {
    let format = match quality {
        EPicQuality::Color(f) | EPicQuality::Default(f) => {
            f.clone()
        },
        EPicQuality::Gray(f) => {
            *img = img.grayscale();
            f.clone()
        },
        EPicQuality::Bitonal(f) => {
            *img = img.grayscale();
            *img = img.adjust_contrast(1000.0);
            f.clone()
        }
    };

    let mut buf: Vec<u8> = vec![];
    match img.write_to(&mut buf, format.clone()) {
        Ok(_) => (),
        Err(e) => return Err(WifError::internal_error(format!("Could not write DynamicImage to buffer --- {:?}", e)))
    }

    Ok((buf, format))
}

impl FromStr for EPicQuality {
    type Err = WifError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() > 1 {
            let format = match &parts[1].to_lowercase() as &str {
                "jpg" | "jpeg" => ImageOutputFormat::Jpeg(config::jpg_quality()),
                "png" => ImageOutputFormat::Png,
                "bmp" => ImageOutputFormat::Bmp,
                "ico" => ImageOutputFormat::Ico,
                "tga" => ImageOutputFormat::Tga,
                _ => return Err(WifError::bad_request("Cannot parse format".to_owned()))
            };

            match parts[0] {
                "color" => return Ok(EPicQuality::Color(format)),
                "gray" => return Ok(EPicQuality::Gray(format)),
                "bitonal" => return Ok(EPicQuality::Bitonal(format)),
                "default" => return Ok(EPicQuality::Default(format)),
                _ => return Err(WifError::bad_request("Cannot parse quality".to_owned()))
            }
        } else {
            return Err(WifError::bad_request("Cannot parse quality and/or format".to_owned()))
        }
    }
}
