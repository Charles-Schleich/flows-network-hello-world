use flowsnet_platform_sdk::logger;
use lambda_flows::{request_received, send_response};

use image::{write_buffer_with_format, ColorType, EncodableLayout, ImageOutputFormat};
use serde_json::Value;
use std::collections::HashMap;
use std::io::{BufWriter, Cursor};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    request_received(|headers, qry: HashMap<String, Value>, body| handler(headers, qry, body))
        .await;
    Ok(())
}

async fn handler(headers: Vec<(String, String)>, _qry: HashMap<String, Value>, _body: Vec<u8>) {
    logger::init();
    log::info!("Headers -- {:?}", headers);

    // let msg = qry.get("msg").unwrap();

    // let resp = format!("Testing Flows Network: This is your message {msg}");
    let fractal_bytes = match generate_fractal(){
        Ok(bytes) => bytes,
        Err(err) => err.as_bytes().to_vec(),
    };

    send_response(
        200,
        vec![(String::from("content-type"), String::from("text/html"))],
        fractal_bytes,
    );
}

fn generate_fractal() -> Result<Vec<u8>, String> {
    const IMG_X: u32 = 800;
    const IMG_Y: u32 = 800;

    let scalex = 3.0 / IMG_X as f32;
    let scaley = 3.0 / IMG_Y as f32;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(IMG_X, IMG_Y);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }

    // A redundant loop to demonstrate reading image data
    for x in 0..IMG_X {
        for y in 0..IMG_Y {
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

    let buffer = [0u8; IMG_X as usize * IMG_Y as usize];
    let cursor_buffer = Cursor::new(buffer); // needed to implement Seek
    let mut stream = BufWriter::new(cursor_buffer);
    
    match write_buffer_with_format(
        &mut stream,
        imgbuf.as_bytes(),
        IMG_X,
        IMG_Y,
        ColorType::Rgb8,
        ImageOutputFormat::Png,
    ){
        Ok(_) => Ok(stream.buffer().to_vec()),
        Err(err) => Err(format!("Error writing Image {err}")),
    }

}
