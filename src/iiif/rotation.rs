use std::str::FromStr;
use super::parsing_error::{ErrorReturnType};
use super::img_container::ImgContainer;

#[derive(Debug)]
pub struct EPicRotation {
    rotation: u32,
    mirrored: bool
}

pub fn mutate_image_rotation(rotation: &EPicRotation, img: &mut ImgContainer) -> Result<(), ErrorReturnType> {
    if rotation.mirrored {
        img.img = img.img.fliph();
    }

    match rotation.rotation {
        0 | 360 => return Ok(()),
        90 => {
            img.img = img.img.rotate90();
        },
        180 => {
            img.img = img.img.rotate180();
        },
        270 => {
            img.img = img.img.rotate270();
        },
        _ => {
            return Err(ErrorReturnType::BadRequest("Rotation must be 0, 90, 180, 270 or 360".to_owned()))
        }
    }

    Ok(())
}


impl FromStr for EPicRotation {
    type Err = ErrorReturnType;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('!').collect();

        if parts.len() > 1 {
            match parts[1].parse::<u32>() {
                Ok(u) => return Ok(Self {
                    rotation: u,
                    mirrored: true
                }),
                Err(_) => return Err(ErrorReturnType::BadRequest("Rotation cannot be parsed".to_owned()))
            }
        } else {
            match parts[0].parse::<u32>() {
                Ok(u) => return Ok(Self {
                    rotation: u,
                    mirrored: false
                }),
                Err(_) => return Err(ErrorReturnType::BadRequest("Rotation cannot be parsed".to_owned()))
            }
        }
    }
}
