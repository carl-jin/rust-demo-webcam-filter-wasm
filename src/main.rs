use std::io::Cursor;

use image::{
    DynamicImage, EncodableLayout, GenericImage, GenericImageView, GrayAlphaImage, GrayImage,
    ImageFormat,
};
use js_sys::{ArrayBuffer, Uint8Array};
use std::time::{Duration, Instant};
use wasm_bindgen::{convert::IntoWasmAbi, prelude::*};
use wasm_bindgen::{Clamped, JsCast};
use web_sys::{self, console, ImageData};

fn main() {
    let width = 1280;
    let height = 720;
    let mut imgDyn = image::DynamicImage::new_rgba8(width, height);
    let length = width * height;
    let yLen = length / width;
    let xLen = length / height;

    let data: Vec<u8> = vec![200; width as usize * height as usize * 4];

    let start = Instant::now();
    for y in 0..yLen {
        for x in 0..xLen {
            let slicePosition = y as usize * width as usize + x as usize;
            let red = data.get(slicePosition + 0).unwrap();
            let green = data.get(slicePosition + 1).unwrap();
            let blue = data.get(slicePosition + 2).unwrap();
            let alpha = data.get(slicePosition + 3).unwrap();

            let pixel = image::Rgba([*red, *green, *blue, *alpha]);
            imgDyn.put_pixel(x, y, pixel);
        }
    }

    // let imgDyn = imgDyn.blur(2.4);

    let output: &[u8] = &*imgDyn.to_rgba8().to_vec();

    // let output =
    //     web_sys::ImageData::new_with_u8_clamped_array_and_sh(Clamped(&output), width, height)
    //         .unwrap();

    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
