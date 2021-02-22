#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
extern crate pretty_env_logger;

use std::str::FromStr;
use std::path::Path;

use percent_encoding::percent_decode;
use warp::{Filter, compression, http::Uri, redirect};

mod responder_wrapper;
use responder_wrapper::ResponderWrapper;
mod iiif;
use iiif::{
    quality::EPicQuality,
    region::EPicRegion,
    rotation::EPicRotation,
    size::EPicSize,
    info_json::IIIFInfo,
    img_container::ImgContainer
};
mod config;
use image::ImageOutputFormat;


lazy_static! {
    static ref IIIF_EXTENSIONS: [(&'static str, ImageOutputFormat); 3] = [
        ("jpg", ImageOutputFormat::Jpeg(config::jpg_quality())),
        ("png", ImageOutputFormat::Png),
        ("bmp", ImageOutputFormat::Bmp)
    ];
}

pub fn render_output(res: ResponderWrapper) -> impl warp::Reply {
    warp::http::Response::builder()
        .status(res.status_code())
        .header("Access-Control-Allow-Origin", "*")
        .header("Content-Type", res.content_type)
        .body(res.content)
}

fn get_iiif_file(identifier: &str) -> Option<(ImageOutputFormat, String)> {
    let path = format!("./{}/{}", config::image_path(), identifier);
    for (ext, format) in IIIF_EXTENSIONS.iter() {
        let ext_path_lower = format!("{}.{}", path, ext);
        if Path::new(&ext_path_lower).exists() {
            return Some((format.clone(), ext_path_lower))
        }
        let ext_path_upper = format!("{}.{}", path, ext.to_uppercase());
        if Path::new(&ext_path_upper).exists() {
            return Some((format.clone(), ext_path_upper))
        }
    }
    None
}

#[tokio::main]
async fn main() {
    ::std::env::set_var("RUST_LOG", "trace");

    pretty_env_logger::init();
    info!("Wif starting up...");

    // Check if the specified img/file folder exists. If not, try to create one.
    if !Path::new(&config::image_path()).exists() {
        match std::fs::create_dir_all(config::image_path()) {
            Ok(_) => (),
            Err(e) => {
                log::error!("{:?}", e);
                panic!(e)
            }
        }
    }

    let cors = warp::cors()
        .allow_any_origin()
        .allow_method("GET");

    let favicon = warp::path("favicon.ico")
        .and(warp::fs::file("./favicon.ico"))
        .with(&cors);

    let iiif_img = warp::path!("iiif" / String / String / String / String / String)
        .map(|identifier_str: String, region_str: String, size_str: String, rotation_str: String, quality_and_format_str: String| {
            let identifier_dec = match percent_decode(identifier_str.as_bytes()).decode_utf8() {
                Ok(v) => v.to_string(),
                Err(_) => return render_output(ResponderWrapper::bad_request("Identifier is malformed"))
            };
            let region_dec = match percent_decode(region_str.as_bytes()).decode_utf8() {
                Ok(v) => v.to_string(),
                Err(_) => return render_output(ResponderWrapper::bad_request("Region is malformed"))
            };
            let size_dec = match percent_decode(size_str.as_bytes()).decode_utf8() {
                Ok(v) => v.to_string(),
                Err(_) => return render_output(ResponderWrapper::bad_request("Size is malformed"))
            };
            let rotation_dec = match percent_decode(rotation_str.as_bytes()).decode_utf8() {
                Ok(v) => v.to_string(),
                Err(_) => return render_output(ResponderWrapper::bad_request("Rotation is malformed"))
            };
            let quality_and_format_dec = match percent_decode(quality_and_format_str.as_bytes()).decode_utf8() {
                Ok(v) => v.to_string(),
                Err(_) => return render_output(ResponderWrapper::bad_request("Quality and/or format are malformed"))
            };

            let file = match get_iiif_file(&identifier_dec) {
                Some(v) => v,
                None => return render_output(ResponderWrapper::not_found(&format!("Image with identifier '{}' not found.", identifier_str)))
            };

            let region = match EPicRegion::from_str(&region_dec) {
                Ok(r) => r,
                Err(e) => return render_output(e.get_response())
            };

            let size = match EPicSize::from_str(&size_dec) {
                Ok(s) => s,
                Err(e) => return render_output(e.get_response())
            };

            let rotation = match EPicRotation::from_str(&rotation_dec) {
                Ok(r) => r,
                Err(e) => return render_output(e.get_response())
            };

            let mut quality = match EPicQuality::from_str(&quality_and_format_dec) {
                Ok(q) => q,
                Err(e) => return render_output(e.get_response())
            };

            let mut img_container = match ImgContainer::from_file(&file.1) {
                Ok(v) => v,
                Err(e) => return render_output(e.get_response())
            };

            match iiif::region::mutate_image_region(&region, &mut img_container) {
                Ok(_) => (),
                Err(e) => return render_output(e.get_response())
            };

            match iiif::size::mutate_image_size(&size, &mut img_container) {
                Ok(_) => (),
                Err(e) => return render_output(e.get_response())
            }

            match iiif::rotation::mutate_image_rotation(&rotation, &mut img_container) {
                Ok(_) => (),
                Err(e) => return render_output(e.get_response())
            }

            match iiif::quality::mutate_image_quality(&mut quality, &mut img_container) {
                Ok(f) => match img_container.body(f.clone()) {
                    Ok(v) => render_output(ResponderWrapper::img(v, &f)),
                    Err(e) => render_output(e.get_response())
                },
                Err(e) => render_output(e.get_response())
            }
        });

    let iiif_json = warp::path!("iiif" / String / "info.json")
        .map(|identifier_str: String| {
            let filepath = match get_iiif_file(&identifier_str) {
                Some(v) => v.1,
                None => return render_output(ResponderWrapper::not_found(&format!("Image with identifier '{}' not found.", &identifier_str)))
            };

            match IIIFInfo::for_img(&filepath, &identifier_str) {
                Ok(v) => render_output(ResponderWrapper::ldjson_text(v)),
                Err(e) => render_output(e.get_response())
            }
        })
        .with(compression::gzip());

    let iiif_baseuri_redirect = warp::path!("iiif" / String)
        .map(|identifier_str: String| {
            redirect(format!("/iiif/{}/info.json", identifier_str).parse::<Uri>().unwrap())
        });

    let default = warp::path::end()
        .map(|| format!("Welcome at Wif! :-)"));

    let routes = iiif_json
                    .or(iiif_baseuri_redirect)
                    .or(iiif_img)
                    .or(favicon)
                    .or(default);


    if config::ssl_enabled() {
        warp::serve(routes)
            .tls()
            .cert_path(config::ssl_cert())
            .key_path(config::ssl_key())
            .run(config::address())
            .await;
    } else {
        warp::serve(routes)
            .run(config::address())
            .await;
    }
}