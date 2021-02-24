use image::ImageFormat;
use lazy_static::lazy_static;
use log::info;
use std::{fs::File, path::Path, io::BufReader};
use png;
use jpeg_decoder;

use crate::{config, wif_error::WifError};

lazy_static! {
    static ref IIIF_EXTENSIONS: [(&'static str, ImageFormat); 3] = [
        ("jpg", ImageFormat::Jpeg),
        ("png", ImageFormat::Png),
        ("bmp", ImageFormat::Bmp)
    ];
}


#[derive(Debug)]
pub struct ImgView {
    pub identifier: String,
    pub filepath: String,
    pub format: ImageFormat,
    pub dimensions: (u32, u32)
}
impl ImgView {
    pub fn for_identifier(identifier: &str) -> Result<Self, WifError> {
        let path = format!("{}/{}", config::image_path(), identifier);
        for (ext, f) in IIIF_EXTENSIONS.iter() {
            let ext_path_lower = format!("{}.{}", path, ext);
            info!("{}", ext_path_lower);
            if Path::new(&ext_path_lower).exists() {
                return Ok(ImgView {
                    dimensions: Self::get_dimensions(&ext_path_lower, f)?,
                    identifier: identifier.to_owned(),
                    filepath: ext_path_lower,
                    format: f.to_owned(),
                })
            }

            let ext_path_upper = format!("{}.{}", path, ext.to_uppercase());
            if Path::new(&ext_path_upper).exists() {
                return Ok(ImgView {
                    dimensions: Self::get_dimensions(&ext_path_lower, f)?,
                    identifier: identifier.to_owned(),
                    filepath: ext_path_upper,
                    format: f.to_owned()
                })
            }
        }

        Err(WifError::not_found(format!("{} not found", identifier)))
    }

    fn get_dimensions(path: &str, format: &ImageFormat) -> Result<(u32, u32), WifError> {
        let reader = match File::open(path) {
            Ok(f) => f,
            Err(e) => {
                log::error!("{:?}", e);
                return Err(WifError::internal_error("Internal Server Error".to_owned()))
            }
        };

        match format {
            ImageFormat::Png => {
                let decoder = png::Decoder::new(reader);
                match decoder.read_info() {
                    Ok(i) => {
                        Ok((i.0.width, i.0.height))
                    },
                    Err(e) => {
                        log::error!("{:?}", e);
                        Err(WifError::internal_error("Internal Server Error".to_owned()))
                    }
                }
            },
            ImageFormat::Jpeg => {
                let mut decoder = jpeg_decoder::Decoder::new(BufReader::new(reader));
                match decoder.read_info() {
                    Ok(_) => {
                        match decoder.info() {
                            Some(i) => Ok((i.width as u32, i.height as u32)),
                            None => Err(WifError::internal_error("Internal Server Error".to_owned()))
                        }
                    },
                    Err(e) => {
                        log::error!("{:?}", e);
                        Err(WifError::internal_error("Internal Server Error".to_owned()))
                    }
                }
            },
            _ => Err(WifError::internal_error("Internal Server Error".to_owned()))
        }
    }
}
