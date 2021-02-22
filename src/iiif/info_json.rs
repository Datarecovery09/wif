use serde::{Serialize};
use super::parsing_error::ErrorReturnType;
use std::path::Path;
use crate::config;

#[derive(Debug, Serialize)]
pub struct IIIFInfo {
    id: String,
    protocol: String,
    profile: String,
    width: u32,
    height: u32,
    max_area: u64,
    preferred_formats: Vec<String>,
    extra_features: Vec<String>,
    extra_qualities: Vec<String>
}

impl IIIFInfo {
    pub fn for_img(img_path: &str, identifier: &str) -> Result<String, ErrorReturnType> {
        if !Path::new(img_path).exists() {
            return Err(ErrorReturnType::NotFound("File not found".to_owned()))
        }

        let dimensions = Self::get_img_info(img_path)?;

        let info = IIIFInfo {
            id: format!("{}/iiif/{}", config::base_address(), identifier),
            protocol: "http://iiif.io/api/image".to_owned(),
            profile: "level1".to_owned(),
            width: dimensions.0,
            height: dimensions.1,
            max_area: dimensions.0 as u64 * dimensions.1 as u64 * 10,
            preferred_formats: vec![
                "tga".to_owned(),
                "png".to_owned(),
                "jpeg".to_owned(),
                "ico".to_owned(),
                "bmp".to_owned()
            ],
            extra_features: vec![
                "baseUriRedirect".to_owned(),
                "rotationBy90s".to_owned(),
                "cors".to_owned(),
                "mirroring".to_owned(),
                "regionByPct".to_owned(),
                "regionByPx".to_owned(),
                "regionSquare".to_owned(),
                "sizeByH".to_owned(),
                "sizeByPct".to_owned(),
                "sizeByW".to_owned(),
                "sizeUpscaling".to_owned()
            ],
            extra_qualities: vec![
                "default".to_owned(),
                "bitonal".to_owned(),
                "gray".to_owned()
            ]
        };

        Ok(format!("{{\"@context\":\"http://iiif.io/api/image/3/context.json\",{}}}", info.jsonify()))
    }

    fn jsonify(&self) -> String {
        format!("\"id\":\"{}\",\"type\":\"ImageService3\",\"protocol\":\"{}\",\"profile\":\"{}\",\"width\":{},\"height\":{},\"maxArea\":{},\"preferredFormats\":{:?},\"extraFeatures\":{:?},\"extraQualities\":{:?}", self.id, self.protocol,self.profile,self.width,self.height,self.max_area,self.preferred_formats,self.extra_features,self.extra_qualities)
    }

    fn get_img_info(img_path: &str) -> Result<(u32, u32), ErrorReturnType> {
        match image::open(img_path) {
            Ok(v) => Ok(v.to_rgb16().dimensions()),
            Err(e) => Err(ErrorReturnType::InternalError(format!("{:?}", e)))
        }
    }
}
