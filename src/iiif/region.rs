use std::str::FromStr;
use image::GenericImageView;

use super::{img_container::ImgContainer, parsing_error::{ErrorReturnType}};

#[derive(Debug)]
pub enum EPicRegion {
    Full,
    Square,
    Reg { x: f32, y: f32, w: f32, h: f32 },
    RegPerc { x: f32, y: f32, w: f32, h: f32 },
}

impl FromStr for EPicRegion {
    type Err = ErrorReturnType;
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
                None => return Err(ErrorReturnType::BadRequest("Request string must contain parameters for coordinates, width and height in the format x,y,w,h.".to_owned()))
            };

            if p.len() > 4 {
                return Err(ErrorReturnType::BadRequest("Request string must not contain more than x,y,w,h parameters.".to_owned()))
            }

            x = match p.get(0) {
                Some(v) => match v.parse::<f32>() {
                    Ok(f) => f,
                    Err(_) => return Err(ErrorReturnType::BadRequest("Request string must contain parameters for coordinates, width and height in the format x,y,w,h.".to_owned()))
                },
                None => return Err(ErrorReturnType::BadRequest("Request string must contain parameters for coordinates, width and height in the format x,y,w,h.".to_owned()))
            };
            y = match p.get(1) {
                Some(v) => match v.parse::<f32>() {
                    Ok(f) => f,
                    Err(_) => return Err(ErrorReturnType::BadRequest("Request string must contain parameters for coordinates, width and height in the format x,y,w,h.".to_owned()))
                },
                None => return Err(ErrorReturnType::BadRequest("Request string must contain parameters for coordinates, width and height in the format x,y,w,h.".to_owned()))
            };
            w = match p.get(2) {
                Some(v) => match v.parse::<f32>() {
                    Ok(f) => f,
                    Err(_) => return Err(ErrorReturnType::BadRequest("Request string must contain parameters for coordinates, width and height in the format x,y,w,h.".to_owned()))
                },
                None => return Err(ErrorReturnType::BadRequest("Request string must contain parameters for coordinates, width and height in the format x,y,w,h.".to_owned()))
            };
            h = match p.get(3) {
                Some(v) => match v.parse::<f32>() {
                    Ok(f) => f,
                    Err(_) => return Err(ErrorReturnType::BadRequest("Request string must contain parameters for coordinates, width and height in the format x,y,w,h.".to_owned()))
                },
                None => return Err(ErrorReturnType::BadRequest("Request string must contain parameters for coordinates, width and height in the format x,y,w,h.".to_owned()))
            };

            if w == 0.0 || h == 0.0 {
                return Err(ErrorReturnType::BadRequest("Width and Height are not allowed to be 0.".to_owned()))
            }

            if x >= 100.0 || y >= 100.0 {
                return Err(ErrorReturnType::BadRequest("Image region must be within the boundaries of the image!.".to_owned()))
            }

            Ok(EPicRegion::RegPerc{x, y, w, h})
        } else {
            p = match pct.get(0) {
                Some(v) => v.split(',').collect::<Vec<&str>>(),
                None => return Err(ErrorReturnType::BadRequest("Request string must contain parameters for coordinates, width and height in the format x,y,w,h.".to_owned()))
            };

            if p.len() > 4 {
                return Err(ErrorReturnType::BadRequest("Request string must not contain more than x,y,w,h parameters.".to_owned()))
            }

            x = match p.get(0) {
                Some(v) => match v.parse::<f32>() {
                    Ok(f) => f,
                    Err(_) => return Err(ErrorReturnType::BadRequest("Request string must contain parameters for coordinates, width and height in the format x,y,w,h.".to_owned()))
                },
                None => return Err(ErrorReturnType::BadRequest("Request string must contain parameters for coordinates, width and height in the format x,y,w,h.".to_owned()))
            };
            y = match p.get(1) {
                Some(v) => match v.parse::<f32>() {
                    Ok(f) => f,
                    Err(_) => return Err(ErrorReturnType::BadRequest("Request string must contain parameters for coordinates, width and height in the format x,y,w,h.".to_owned()))
                },
                None => return Err(ErrorReturnType::BadRequest("Request string must contain parameters for coordinates, width and height in the format x,y,w,h.".to_owned()))
            };
            w = match p.get(2) {
                Some(v) => match v.parse::<f32>() {
                    Ok(f) => f,
                    Err(_) => return Err(ErrorReturnType::BadRequest("Request string must contain parameters for coordinates, width and height in the format x,y,w,h.".to_owned()))
                },
                None => return Err(ErrorReturnType::BadRequest("Request string must contain parameters for coordinates, width and height in the format x,y,w,h.".to_owned()))
            };
            h = match p.get(3) {
                Some(v) => match v.parse::<f32>() {
                    Ok(f) => f,
                    Err(_) => return Err(ErrorReturnType::BadRequest("Request string must contain parameters for coordinates, width and height in the format x,y,w,h.".to_owned()))
                },
                None => return Err(ErrorReturnType::BadRequest("Request string must contain parameters for coordinates, width and height in the format x,y,w,h.".to_owned()))
            };

            if w == 0.0 || h == 0.0 {
                return Err(ErrorReturnType::BadRequest("Width and Height are not allowed to be 0.".to_owned()))
            }

            Ok(EPicRegion::Reg{x, y, w, h})
        }
    }
}


pub fn mutate_image_region(region: &EPicRegion, img: &mut ImgContainer) -> Result<(), ErrorReturnType> {
    match region {
        EPicRegion::Full => (),
        EPicRegion::Square => {
            let dim = img.img.dimensions();
            if img.img.width() > img.img.height() {
                img.img = image::DynamicImage::crop(&mut img.img, (dim.0 - dim.1)/2, 0, dim.1, dim.1);
            } else if img.img.width() < img.img.height() {
                img.img = image::DynamicImage::crop(&mut img.img, 0, (dim.1 - dim.0)/2, dim.0, dim.0);
            }
        },
        EPicRegion::Reg {x, y, w, h} => {
            match rect_in_bounds(img.img.dimensions(), x.round() as u32, y.round() as u32, w.round() as u32, h.round() as u32) {
                (true, xp, yp, hp, wp) => {
                    img.img = image::DynamicImage::crop(&mut img.img, xp, yp, wp, hp);
                },
                (false, _, _, _, _) => {
                    return Err(ErrorReturnType::BadRequest("Region is out of bounds".to_owned()))
                }
            };
        },
        EPicRegion::RegPerc {x, y, w, h} => {
            match rect_in_bounds_perc(img.img.dimensions(), x, y, w, h) {
                (true, xp, yp, wp, hp) => {
                    img.img = image::DynamicImage::crop(&mut img.img, xp, yp, wp, hp);
                },
                (false, _, _, _, _) => {
                    return Err(ErrorReturnType::BadRequest("Region is out of bounds".to_owned()))
                }
            }
        }
    }

    Ok(())
}

fn rect_in_bounds(dim: (u32, u32), x: u32, y: u32, w: u32, h: u32) -> (bool, u32, u32, u32, u32) {
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
fn rect_in_bounds_perc(dim: (u32, u32), x: &f32, y: &f32, w: &f32, h: &f32) -> (bool, u32, u32, u32, u32) {
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
