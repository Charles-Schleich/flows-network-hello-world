use flowsnet_platform_sdk::logger;
use lambda_flows::{request_received, send_response};

use image::ImageOutputFormat;
use serde_json::Value;
use std::collections::HashMap;
use std::io::Cursor;
use std::str::FromStr;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    request_received(|headers, qry: HashMap<String, Value>, body| handler(headers, qry, body))
        .await;
    Ok(())
}

/// Handler Function for Http Requests
/// Expecting: re:f64, im:f64
/// Optional : dim:u64
/// example: http://code.flows.network/lambda/<TOKEN>?re=-0.55&im=0.55&dim=100

async fn handler(headers: Vec<(String, String)>, qry: HashMap<String, Value>, _body: Vec<u8>) {
    logger::init();
    log::info!("Headers -- {:?}", headers);

    fn url_param<T: FromStr>(query_name: &str, qry: &HashMap<String, Value>) -> Option<T> {
        qry.get(query_name)
            .and_then(|x| x.as_str())
            .and_then(|x| x.parse().ok())
    }

    let mut dim = url_param::<u32>("dim", &qry).unwrap_or(500) as u32;
    // Requests become too long for large images
    // Also large images can generate large amounts of data
    if dim > 5000 {
        dim = 5000;
    }

    let (re, im) = match (url_param::<f64>("re", &qry), url_param::<f64>("im", &qry)) {
        (Some(re), Some(im)) => (re, im),
        _ => {
            send_response(
                400,
                vec![(String::from("content-type"), String::from("text/html"))],
                "Expecting request to have params for real and imaginary values \n e.g. url?re=-0.55&im=0.55".to_string().as_bytes().to_vec(),
            );
            return;
        }
    };

    // Generate Fractal
    let fractal_bytes = match generate_fractal(dim, re, im) {
        Ok(bytes) => bytes,
        Err(err) => err.as_bytes().to_vec(),
    };

    send_response(
        200,
        vec![(String::from("content-type"), String::from("image/png"))],
        fractal_bytes,
    );
}

pub fn generate_fractal(img_dim: u32, re: f64, im: f64) -> Result<Vec<u8>, String> {
    let scale = 3.0 / img_dim as f64;

    // Create a new ImgBuf with width and height: img_dim
    let mut imgbuf = image::ImageBuffer::new(img_dim, img_dim);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let b = (0.3 * y as f64) as u8;

        let cx: f64 = y as f64 * scale - 1.5;
        let cy: f64 = x as f64 * scale - 1.5;
        let c = num_complex::Complex::new(re, im);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut g = 0;
        while g < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            g += 1;
        }
        let r: u8 = g;

        *pixel = image::Rgb([r, g, b]);
    }

    let vec = Vec::new();
    // Cursor needed to implement Seek
    let mut cursor_buffer = Cursor::new(vec);

    match imgbuf.write_to(&mut cursor_buffer, ImageOutputFormat::Png) {
        Ok(_) => Ok(cursor_buffer.into_inner()),
        Err(err) => Err(format!("Error writing Image {err}")),
    }
}
