use flowsnet_platform_sdk::logger;
use lambda_flows::{request_received, send_response};

use serde_json::Value;
use std::collections::HashMap;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {

    request_received(|headers, qry: HashMap<String, Value>, body| handler(headers, qry, body)).await;
    Ok(())
}

async fn handler(
    headers: Vec<(String, String)>,
    qry: HashMap<String, Value>,
    _body: Vec<u8>,
) {
    logger::init();
    log::info!("Headers -- {:?}", headers);

    let msg = qry.get("msg").unwrap();

    let resp = format!(
        "Testing Flows Network: This is your message {msg}"
    );

    send_response(
        200,
        vec![(String::from("content-type"), String::from("text/html"))],
        resp.as_bytes().to_vec(),
    );
}

use image::{ImageBuffer, EncodableLayout};
fn generate_fractal(){
    let imgx = 800;
    let imgy = 800;

    let scalex = 3.0 / imgx as f32;
    let scaley = 3.0 / imgy as f32;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }

    // A redundant loop to demonstrate reading image data
    for x in 0..imgx {
        for y in 0..imgy {
            let cx = y as f32 * scalex - 1.5;
            let cy = x as f32 * scaley - 1.5;

            let c = num_complex::Complex::new(-0.4, 0.6);
            let mut z = num_complex::Complex::new(cx, cy);

            let mut i = 0;
            while i < 255 && z.norm() <= 2.0 {
                z = z * z + c;
                i += 1;
            }

            let pixel = imgbuf.get_pixel_mut(x, y);
            let image::Rgb(data) = *pixel;
            *pixel = image::Rgb([data[0], i as u8, data[2]]);
        }
    }

    let image_buff_bytes = imgbuf.as_bytes().to_vec();

}