use std::str::FromStr;

use image::{ImageFormat, ImageOutputFormat};
use log::info;
use pretty_env_logger;

use tide::{Body, Request, Response, StatusCode, http::mime, utils::After};

mod wif_error;
use wif_error::WifError;
mod iiif;
use iiif::{
    img_info::ImgView,
    info_json::IIIFInfo,
    region::EPicRegion,
    size::EPicSize,
    rotation::EPicRotation,
    quality::EPicQuality
};
mod config;


#[async_std::main]
async fn main() -> tide::Result<()> {
    ::std::env::set_var("RUST_LOG", "trace");

    pretty_env_logger::init();
    info!("Wif starting up...");

    let mut app = tide::new();

    app.with(After(|mut res: Response| async {
        if let Some(err) = res.downcast_error::<WifError>() {
            let status = err.status;
            let msg = err.message.clone();
            res.set_status(status);
            res.set_body(msg);
        }
        Ok(res)
    }));

    app.with(tide_compress::CompressMiddleware::new());

    app.at("/").get(|_| async {
        Ok("Welcome at Wif! :-)")
    });
    app.at("/favicon.ico").get(|_| async {
        Ok(Body::from_file("./favicon.ico").await?)
    });
    app.at("/iiif/:identifier").get(redirect_info_json);
    app.at("/iiif/:identifier/info.json").get(info_json);
    app.at("/iiif/:identifier/:region/:size/:rotation/:quality").get(show_img);
    app.listen(config::address_as_str()).await?;

    Ok(())
}

async fn show_img(req: Request<()>) -> tide::Result<Response> {
    let img_identifier = req.param("identifier")?;
    
    let img_info = ImgView::for_identifier(img_identifier)?;
    let region = EPicRegion::from_str(req.param("region")?)?;
    let size = EPicSize::from_str(req.param("size")?)?;
    let rotation = EPicRotation::from_str(req.param("rotation")?)?;
    let mut quality = EPicQuality::from_str(req.param("quality")?)?;

    match try_stream_unmodified(&img_info, &region, &size, &rotation, &quality).await {
        Some(v) => return Ok(v),
        None => ()
    }

    let mut img = region.from_file(&img_info)?;
    iiif::size::mutate_image_size(&size, &mut img)?;
    iiif::rotation::mutate_image_rotation(&rotation, &mut img)?;
    let buffer = iiif::quality::mutate_image_quality(&mut quality, &mut img)?;

    let mimetype = match buffer.1 {
        ImageOutputFormat::Png => mime::PNG,
        ImageOutputFormat::Jpeg(_) => mime::JPEG,
        _ => mime::BYTE_STREAM
    };

    let status = StatusCode::Ok;
    let mut res = Response::new(status);
    res.set_content_type(mimetype);
    res.set_body(buffer.0);
    Ok(res)
}


async fn redirect_info_json(req: Request<()>) -> tide::Result<Response> {
    let img_path = req.param("identifier")?;

    let mut builder = Response::new(StatusCode::MovedPermanently);
    builder.append_header("Location", format!("/iiif/{}/info.json", img_path));

    Ok(builder)
}

async fn info_json(req: Request<()>) -> tide::Result<Response> {
    let img_name = req.param("identifier")?;
    let img_info = ImgView::for_identifier(img_name)?;
    let info_json = IIIFInfo::for_img(&img_info)?;

    let status = StatusCode::Ok;
    let mut res = Response::new(status);
    res.set_content_type(mime::JSON);
    res.set_body(info_json);
    Ok(res)
}

async fn try_stream_unmodified(img_view: &ImgView, region: &EPicRegion, size: &EPicSize, rotation: &EPicRotation, quality: &EPicQuality) -> Option<Response> {
    match region {
        EPicRegion::Full => (),
        _ => return None
    }

    match size {
        EPicSize::Max => (),
        _ => return None
    }

    if rotation.rotation != 0 || rotation.mirrored {
        return None
    }

    let mime;
    match quality {
        EPicQuality::Default(f) | EPicQuality::Color(f) => {
            match f {
                ImageOutputFormat::Jpeg(_) => {
                    if img_view.format == ImageFormat::Jpeg {
                        mime = mime::JPEG;
                    } else {return None}
                },
                ImageOutputFormat::Png => {
                    if img_view.format == ImageFormat::Png {
                        mime = mime::PNG;
                    } else {return None}
                }
                _ => return None
            }
        },
        _ => return None
    }

    let body = match Body::from_file(&img_view.filepath).await {
        Ok(v) => v,
        Err(e) => {
            log::error!("Error --- {:?}", e);
            return None
        }
    };
    let mut early_resp = Response::new(StatusCode::Ok);
    early_resp.set_content_type(mime);
    early_resp.set_body(body);
    Some(early_resp)
}
