use serde::{Serialize};
use crate::wif_error::WifError;
use crate::config;
use super::img_info::ImgView;

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
    pub fn for_img(img: &ImgView) -> Result<String, WifError> {
        // let dimensions = Self::get_img_info(&img.filepath)?;

        let info = IIIFInfo {
            id: format!("{}/iiif/{}", config::base_address(), &img.identifier),
            protocol: "http://iiif.io/api/image".to_owned(),
            profile: "level1".to_owned(),
            width: img.dimensions.width,
            height: img.dimensions.height,
            max_area: config::max_area(),
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
}
