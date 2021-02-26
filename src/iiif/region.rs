use std::str::FromStr;
use image::{DynamicImage, GenericImageView, ImageBuffer};
use crate::wif_error::WifError;
use super::img_info::{ImgSection, ImgView, Rect};

#[derive(Debug)]
pub enum EPicRegion {
    Full,
    Square,
    Reg { x: f32, y: f32, w: f32, h: f32 },
    RegPerc { x: f32, y: f32, w: f32, h: f32 },
}

impl FromStr for EPicRegion {
    type Err = WifError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "full" => return Ok(EPicRegion::Full),
            "square" => return Ok(EPicRegion::Square),
            _ => ()
        }

        let pct: Vec<&str> = s.split("pct:").collect();
        let p: Vec<&str>;
        let x: f32;
        let y: f32;
        let w: f32;
        let h: f32;
        if pct.len() > 1 {
            p = match pct.get(1) {
                Some(v) => v.split(',').collect::<Vec<&str>>(),
                None => return Err(WifError::bad_request("Request string must contain parameters for coordinates, width and height in the format x,y,w,h.".to_owned()))
            };

            if p.len() > 4 {
                return Err(WifError::bad_request("Request string must not contain more than x,y,w,h parameters.".to_owned()))
            }

            x = match p.get(0) {
                Some(v) => match v.parse::<f32>() {
                    Ok(f) => f,
                    Err(_) => return Err(WifError::bad_request("Request string must contain parameters for coordinates, width and height in the format x,y,w,h.".to_owned()))
                },
                None => return Err(WifError::bad_request("Request string must contain parameters for coordinates, width and height in the format x,y,w,h.".to_owned()))
            };
            y = match p.get(1) {
                Some(v) => match v.parse::<f32>() {
                    Ok(f) => f,
                    Err(_) => return Err(WifError::bad_request("Request string must contain parameters for coordinates, width and height in the format x,y,w,h.".to_owned()))
                },
                None => return Err(WifError::bad_request("Request string must contain parameters for coordinates, width and height in the format x,y,w,h.".to_owned()))
            };
            w = match p.get(2) {
                Some(v) => match v.parse::<f32>() {
                    Ok(f) => f,
                    Err(_) => return Err(WifError::bad_request("Request string must contain parameters for coordinates, width and height in the format x,y,w,h.".to_owned()))
                },
                None => return Err(WifError::bad_request("Request string must contain parameters for coordinates, width and height in the format x,y,w,h.".to_owned()))
            };
            h = match p.get(3) {
                Some(v) => match v.parse::<f32>() {
                    Ok(f) => f,
                    Err(_) => return Err(WifError::bad_request("Request string must contain parameters for coordinates, width and height in the format x,y,w,h.".to_owned()))
                },
                None => return Err(WifError::bad_request("Request string must contain parameters for coordinates, width and height in the format x,y,w,h.".to_owned()))
            };

            if w == 0.0 || h == 0.0 {
                return Err(WifError::bad_request("Width and Height are not allowed to be 0.".to_owned()))
            }

            if x >= 100.0 || y >= 100.0 {
                return Err(WifError::bad_request("Image region must be within the boundaries of the image!.".to_owned()))
            }

            Ok(EPicRegion::RegPerc{x, y, w, h})
        } else {
            p = match pct.get(0) {
                Some(v) => v.split(',').collect::<Vec<&str>>(),
                None => return Err(WifError::bad_request("Request string must contain parameters for coordinates, width and height in the format x,y,w,h.".to_owned()))
            };

            if p.len() > 4 {
                return Err(WifError::bad_request("Request string must not contain more than x,y,w,h parameters.".to_owned()))
            }

            x = match p.get(0) {
                Some(v) => match v.parse::<f32>() {
                    Ok(f) => f,
                    Err(_) => return Err(WifError::bad_request("Request string must contain parameters for coordinates, width and height in the format x,y,w,h.".to_owned()))
                },
                None => return Err(WifError::bad_request("Request string must contain parameters for coordinates, width and height in the format x,y,w,h.".to_owned()))
            };
            y = match p.get(1) {
                Some(v) => match v.parse::<f32>() {
                    Ok(f) => f,
                    Err(_) => return Err(WifError::bad_request("Request string must contain parameters for coordinates, width and height in the format x,y,w,h.".to_owned()))
                },
                None => return Err(WifError::bad_request("Request string must contain parameters for coordinates, width and height in the format x,y,w,h.".to_owned()))
            };
            w = match p.get(2) {
                Some(v) => match v.parse::<f32>() {
                    Ok(f) => f,
                    Err(_) => return Err(WifError::bad_request("Request string must contain parameters for coordinates, width and height in the format x,y,w,h.".to_owned()))
                },
                None => return Err(WifError::bad_request("Request string must contain parameters for coordinates, width and height in the format x,y,w,h.".to_owned()))
            };
            h = match p.get(3) {
                Some(v) => match v.parse::<f32>() {
                    Ok(f) => f,
                    Err(_) => return Err(WifError::bad_request("Request string must contain parameters for coordinates, width and height in the format x,y,w,h.".to_owned()))
                },
                None => return Err(WifError::bad_request("Request string must contain parameters for coordinates, width and height in the format x,y,w,h.".to_owned()))
            };

            if w == 0.0 || h == 0.0 {
                return Err(WifError::bad_request("Width and Height are not allowed to be 0.".to_owned()))
            }

            Ok(EPicRegion::Reg{x, y, w, h})
        }
    }
}

impl EPicRegion {
    pub fn from_file(&self, img_view: &ImgView) -> Result<DynamicImage, WifError> {
        match img_view.format {
            image::ImageFormat::Png => {
                match self.try_get_from_png(img_view) {
                    Some(v) => {
                        return Ok(v)
                    },
                    None => ()
                }
            },
            _ => ()
        }

        let mut dyn_img = match image::open(&img_view.filepath) {
            Ok(v) => v,
            Err(e) => return Err(WifError::internal_error(format!("{:?}", e)))
        };

        self.mutate_image_region(&mut dyn_img)?;

        Ok(dyn_img)
    }

    pub fn mutate_image_region(&self, img: &mut DynamicImage) -> Result<(), WifError> {
        match self {
            EPicRegion::Full => (),
            EPicRegion::Square => {
                let dim = img.dimensions();
                if img.width() > img.height() {
                    *img = image::DynamicImage::crop(img, (dim.0 - dim.1)/2, 0, dim.1, dim.1);
                } else if img.width() < img.height() {
                    *img = image::DynamicImage::crop(img, 0, (dim.1 - dim.0)/2, dim.0, dim.0);
                }
            },
            EPicRegion::Reg {x, y, w, h} => {
                match self.rect_in_bounds(img.dimensions(), x.round() as u32, y.round() as u32, w.round() as u32, h.round() as u32) {
                    (true, xp, yp, hp, wp) => {
                        *img = image::DynamicImage::crop(img, xp, yp, wp, hp);
                    },
                    (false, _, _, _, _) => {
                        return Err(WifError::bad_request("Region is out of bounds".to_owned()))
                    }
                };
            },
            EPicRegion::RegPerc {x, y, w, h} => {
                match self.rect_in_bounds_perc(img.dimensions(), x, y, w, h) {
                    (true, xp, yp, wp, hp) => {
                        *img = image::DynamicImage::crop(img, xp, yp, wp, hp);
                    },
                    (false, _, _, _, _) => {
                        return Err(WifError::bad_request("Region is out of bounds".to_owned()))
                    }
                }
            }
        }
    
        Ok(())
    }
    
    fn rect_in_bounds(&self, dim: (u32, u32), x: u32, y: u32, w: u32, h: u32) -> (bool, u32, u32, u32, u32) {
        if x >= dim.0 || y >= dim.1 {
            return (false, 0, 0, 0, 0)
        }
    
        let mut wp = w;
        let mut hp = h;
    
        if (x + w) > dim.0 {
            wp = dim.0;
        }
        if (y + h) > dim.1 {
            hp = dim.1;
        }
    
        (true, x, y, wp, hp)
    }
    fn rect_in_bounds_perc(&self, dim: (u32, u32), x: &f32, y: &f32, w: &f32, h: &f32) -> (bool, u32, u32, u32, u32) {
        let xp: u32 = (dim.0 as f32 * (x / 100f32)).round() as u32;
        let yp: u32 = (dim.1 as f32 * (y / 100f32)).round() as u32;
        let mut wp: u32 = (dim.0 as f32 * (w / 100f32)).round() as u32;
        let mut hp: u32 = (dim.1 as f32 * (h / 100f32)).round() as u32;
    
        if xp >= dim.0 || yp >= dim.1 {
            return (false, 0, 0, 0, 0)
        }
    
        if (xp + wp) > dim.0 {
            wp = dim.0;
        }
        if (yp + hp) > dim.1 {
            hp = dim.1;
        }
    
        (true, xp, yp, wp, hp)
    }
    

    fn try_get_from_png(&self, img_view: &ImgView) -> Option<DynamicImage> {
        let mut section: Option<ImgSection> = None;
        match self {
            EPicRegion::Square => {
                if img_view.width() > img_view.height() {
                    section = Some(ImgSection {
                        x: img_view.height() /2,
                        y: 0,
                        dimensions: Rect {
                            width: img_view.height(),
                            height: img_view.height()
                        }
                    });
                } else if img_view.width() == img_view.height() {
                    return None
                } else {
                    section = Some(ImgSection {
                        x: 0,
                        y: img_view.width() /2,
                        dimensions: Rect {
                            width: img_view.width(),
                            height: img_view.width()
                        }
                    });
                }
            },

            EPicRegion::Reg {x, y, w, h} => {
                let new_w = if w + x > img_view.width() as f32 {
                    img_view.width() - x.round() as u32
                } else { w.round() as u32 };
                let new_h = if h + y > img_view.height() as f32 {
                    img_view.height() - y.round() as u32
                } else { h.round() as u32 };
                section = Some(ImgSection {
                    x: x.round() as u32,
                    y: y.round() as u32,
                    dimensions: Rect {
                        width: new_w,
                        height: new_h
                    }
                });
            },

            EPicRegion::RegPerc { x, y, w, h } => {
                let new_w = if w + x > img_view.width() as f32 {
                    img_view.width()
                } else { w.round() as u32 };
                let new_h = if h + y > img_view.height() as f32 {
                    img_view.height()
                } else { h.round() as u32 };
                section = Some(ImgSection {
                    x: x.round() as u32,
                    y: y.round() as u32,
                    dimensions: Rect {
                        width: new_w,
                        height: new_h
                    }
                });
            }

            _ => ()
        }

        if let Some(s) = section {
            self.read_png_section(&img_view.filepath, s)
        } else {
            None
        }
    }

    fn read_png_section(&self, filepath: &str, section: ImgSection) -> Option<DynamicImage> {
        let f = match std::fs::File::open(filepath) {
            Ok(v) => v,
            Err(e) => {
                log::error!("{}", e);
                return None
            }
        };

        let decoder = png::Decoder::new(f);
        let (info, mut reader) = match decoder.read_info() {
            Ok(v) => v,
            Err(e) => {
                log::error!("{}", e);
                return None
            }
        };

        let color_multi = match info.color_type {
            png::ColorType::RGB => 3,
            png::ColorType::RGBA => 4,
            png::ColorType::Grayscale => 1,
            png::ColorType::GrayscaleAlpha => 2,
            png::ColorType::Indexed => 1
        };
        let bit_multi = match info.bit_depth {
            png::BitDepth::Eight => 1,
            png::BitDepth::Sixteen => 2,
            _ => 1
        };

        let pixel_size = bit_multi * color_multi;

        let buffer_size = (section.width() * section.height() * pixel_size) as usize;
        let mut buf: Vec<u8> = vec![0; buffer_size];

        let mut len = 0;
        for n in 0..(section.height() + section.y) {
            match reader.next_row() {
                Ok(r) => {
                    if let Some(row) = r {
                        if n < section.y { continue }
                        for i in 0..(section.width() * pixel_size + section.x) as usize {
                            if i < section.x as usize { continue }
                            if let Some(b) = row.get(i) {
                                buf[len] = *b;
                                len += 1;
                            }
                        }
                    }
                },
                Err(e) => {
                    log::error!("{}", e);
                    return None
                }
            }
        }

        match info.color_type {
            png::ColorType::RGB => Some(DynamicImage::ImageRgb8(ImageBuffer::from_raw(section.width(), section.height(), buf)?)),
            png::ColorType::RGBA => Some(DynamicImage::ImageRgba8(ImageBuffer::from_raw(section.width(), section.height(), buf)?)),
            png::ColorType::Grayscale => Some(DynamicImage::ImageLuma8(ImageBuffer::from_raw(section.width(), section.height(), buf)?)),
            png::ColorType::GrayscaleAlpha => Some(DynamicImage::ImageLumaA8(ImageBuffer::from_raw(section.width(), section.height(), buf)?)),
            _ => None
        }
    }
}
