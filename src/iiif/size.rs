use std::str::FromStr;
use image::{DynamicImage, GenericImageView, imageops::{self}};

use crate::wif_error::WifError;

#[derive(Debug)]
pub enum EPicSize {
    Max,
    Width { w: f32, upscale: bool },
    Height { h: f32, upscale: bool },
    Perc { n: f32, upscale: bool },
    WidthHeight { w: f32, h: f32, forced: bool, upscale: bool }
}

impl FromStr for EPicSize {
    type Err = WifError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "max" | "^max" => return Ok(EPicSize::Max),
            _ => ()
        }

        let part: &str;
        let forced: bool;
        let upscale: bool;

        let upscale_parts: Vec<&str> = s.split('^').collect();
        if upscale_parts.len() > 1 {
            upscale = true;
        } else {
            upscale = false;
        }

        let parts: Vec<&str> = if upscale {
            upscale_parts[1].split('!').collect()
        } else {
            upscale_parts[0].split('!').collect()
        };

        if parts.len() > 1 {
            forced = true;
            part = parts[1];
        } else {
            forced = false;
            part = parts[0];
        }

        if let Some(n) = Self::parse_as_percent(part) {
            return Ok(EPicSize::Perc { n, upscale })
        }

        if let Some((w, h)) = Self::parse_as_width_and_height(part) {
            return Ok(EPicSize::WidthHeight { w, h, forced, upscale })
        }

        if let Some(w) = Self::parse_as_width(part) {
            return Ok(EPicSize::Width { w, upscale })
        }

        if let Some(h) = Self::parse_as_height(part) {
            return Ok(EPicSize::Height { h, upscale })
        }

        Err(WifError::bad_request("Cannot parse parameter size".to_owned()))
    }
}

impl EPicSize {
    fn parse_as_width(s: &str) -> Option<f32> {
        let parts: Vec<&str> = s.split(',').collect();

        if let Some(v) = parts.get(0) {
            if let Ok(f) = v.parse::<f32>() {
                return Some(f)
            }
        }
        None
    }

    fn parse_as_height(s: &str) -> Option<f32> {
        let parts: Vec<&str> = s.split(',').collect();

        if let Some(v) = parts.get(1) {
            if let Ok(f) = v.parse::<f32>() {
                return Some(f)
            }
        }
        None
    }

    fn parse_as_percent(s: &str) -> Option<f32> {
        let parts: Vec<&str> = s.split("pct:").collect();

        if let Some(v) = parts.get(1) {
            if let Ok(f) = v.parse::<f32>() {
                return Some(f)
            }
        }
        None
    }

    fn parse_as_width_and_height(s: &str) -> Option<(f32, f32)> {
        let parts: Vec<&str> = s.split(',').collect();

        if let Some(v) = parts.get(0) {
            if let Ok(f1) = v.parse::<f32>() {
                if let Some(b) = parts.get(1) {
                    if let Ok(f2) = b.parse::<f32>() {
                        return Some((f1, f2))
                    }
                }
            }
        }
        None
    }

}

pub fn mutate_image_size(size: &EPicSize, img: &mut DynamicImage) -> Result<(), WifError> {
    match size {
        EPicSize::Max => (),
        EPicSize::Height {h, upscale} => {
            let mut nwidth = img.width();

            if h.round() as u32 > img.height() {
                if *upscale {
                    let multi = h / img.height() as f32;
                    nwidth = (nwidth as f32 * multi).round() as u32;
                } else {
                    return Err(WifError::bad_request("Size not allowed".to_owned()))
                }
            }

            *img = image::DynamicImage::resize(img, nwidth, h.round() as u32, image::imageops::FilterType::CatmullRom);
        },
        EPicSize::Width {w, upscale} => {
            let mut nheight = img.height();

            if w.round() as u32 > img.width() {
                if *upscale {
                    let multi = w / img.width() as f32;
                    nheight = (nheight as f32 * multi).round() as u32;
                } else {
                    return Err(WifError::bad_request("Size not allowed".to_owned()))
                }
            }

            *img = image::DynamicImage::resize(img, w.round() as u32, nheight, image::imageops::FilterType::CatmullRom);
        },
        EPicSize::Perc {n, upscale} => {
            if *n >= 100.0 && !upscale {
                return Err(WifError::bad_request("Size not allowed".to_owned()))
            }

            let m_width = (img.width() as f32 * *n * 0.01).round() as u32;
            let m_height = (img.height() as f32 * *n * 0.01).round() as u32;
            *img = img.resize(m_width, m_height, imageops::FilterType::CatmullRom);
        },
        EPicSize::WidthHeight {w, h, forced, upscale} => {
            let n_w = if *w > img.width() as f32 && !upscale {
                return Err(WifError::bad_request("This size is not allowed".to_owned()))
            } else {
                w.round() as u32
            };
            let n_h = if *h > img.height() as f32 && !upscale {
                return Err(WifError::bad_request("This size is not allowed".to_owned()))
            } else {
                h.round() as u32
            };
            if *forced {
                *img = image::DynamicImage::resize(img, n_w, n_h, image::imageops::FilterType::CatmullRom);
            } else {
                *img = image::DynamicImage::resize_exact(img, n_w, n_h, image::imageops::FilterType::CatmullRom);
            }
        }
    }
    Ok(())
}
