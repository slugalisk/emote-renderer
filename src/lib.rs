mod utils;

use apng;
use apng::Encoder;
use apng::{Frame, PNGImage};
use std::io::BufWriter;
use wasm_bindgen::JsCast;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn initialize() {
    utils::set_panic_hook();
}

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen (js_name = pngDecode)]
pub fn png_decode(data: Vec<u8>, frame_width: u32, frame_height: u32) -> Vec<u8> {
    // let decoder = png::Decoder::new(&data[..]);

    // let (info, mut reader) = decoder.read_info().unwrap_throw();

    // console_log!("img_data size: {}", info.buffer_size());

    // let mut buf = vec![0; info.buffer_size()];
    // reader.next_frame(&mut buf).unwrap();

    // console_log!("this one: {}", reader.info().frame_control.is_some());

    let img = image::load_from_memory_with_format(&data, image::ImageFormat::PNG)
        .unwrap_throw()
        .to_rgba();
    let img_data = img.to_vec();

    // framestrip

    let mut frames: Vec<u8> = vec![0; img_data.len()];

    let frame_count = img.width() / frame_width;
    for y in 0..img.height() {
        for x in 0..frame_count {
            let src_start = y * img.width() * 4 + x * frame_width * 4;
            let src_end = src_start + frame_width * 4;

            let dst_start = x * frame_width * frame_height * 4 + y * frame_width * 4;
            let dst_end = dst_start + frame_width * 4;

            frames[dst_start as usize..dst_end as usize]
                .copy_from_slice(&img_data[src_start as usize..src_end as usize]);
        }
    }

    frames
}

#[wasm_bindgen (js_name = apngEncodeAll)]
pub fn apng_encode_all(
    data: js_sys::Array,
    frame_speed: f64,
    frame_width: u32,
    frame_height: u32,
) -> Vec<u8> {
    let data: Vec<Vec<u8>> = data
        .values()
        .into_iter()
        .map(|x| {
            x.unwrap_throw()
                .unchecked_ref::<js_sys::Uint8Array>()
                .to_vec()
        })
        .collect();

    let mut png_images: Vec<PNGImage> = Vec::new();
    for v in data {
        let img = image::RgbaImage::from_vec(frame_width, frame_height, v).unwrap();
        png_images.push(apng::load_dynamic_image(image::ImageRgba8(img)).unwrap());
    }

    let mut buf = Vec::new();
    {
        let mut buf_writer = BufWriter::new(&mut buf);

        let config = apng::create_config(&png_images, None).unwrap();
        let mut encoder = Encoder::new(&mut buf_writer, config).unwrap();
        let d_num = frame_speed * (100 as f64);
        let d_den = 100;

        let frame = Frame {
            delay_num: Some(d_num as u16),
            delay_den: Some(d_den),
            ..Default::default()
        };

        match encoder.encode_all(png_images, Some(&frame)) {
            Ok(_n) => log("success apng encode!!!"),
            Err(err) => console_log!("{}", err),
        }
    }

    buf
}
