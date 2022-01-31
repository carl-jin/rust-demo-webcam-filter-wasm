use std::fmt::Error;
use std::io::Cursor;

use image::{
    DynamicImage, EncodableLayout, GenericImage, GenericImageView, GrayImage, ImageFormat,
};
use js_sys::{ArrayBuffer, Uint8Array};
use qrcode::QrCode;
use wasm_bindgen::{convert::IntoWasmAbi, prelude::*};
use wasm_bindgen::{Clamped, JsCast};
use web_sys::{self, console, ImageData};

#[wasm_bindgen(start)]
pub fn init_start() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct VideoFilter {
    inputCanvas: web_sys::HtmlCanvasElement,
    outputCanvas: web_sys::HtmlCanvasElement,
    qr: DynamicImage,
}

fn gen_qrcode(url: &str) -> DynamicImage {
    let code = QrCode::new(url.as_bytes()).unwrap();
    let imgBuff = code.render::<image::Luma<u8>>().build();
    DynamicImage::ImageLuma8(imgBuff)
}

#[wasm_bindgen]
impl VideoFilter {
    #[wasm_bindgen(constructor)]
    pub fn new(
        inputCanvas: web_sys::HtmlCanvasElement,
        outputCanvas: web_sys::HtmlCanvasElement,
    ) -> Self {
        Self {
            inputCanvas,
            outputCanvas,
            qr: gen_qrcode(
                "https://github.com/carl-jin/rust-demo-web2image/blob/main/src/web2image.rs",
            ),
        }
    }

    #[wasm_bindgen(method)]
    pub fn render(&mut self) -> Result<(), JsValue> {
        let width = self.inputCanvas.width() as u32;
        let height = self.inputCanvas.height() as u32;
        let input_canvas_context = self
            .inputCanvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
        let output_canvas_context = self
            .outputCanvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

        let input_image_data =
            input_canvas_context.get_image_data(0.0, 0.0, width as f64, height as f64)?;

        output_canvas_context.put_image_data(
            &self.imageFilter(&input_image_data, width, height),
            0.0,
            0.0,
        )?;

        Ok(())
    }

    fn imageFilter(&self, image: &ImageData, width: u32, height: u32) -> ImageData {
        let mut imgData = image.data().to_vec();
        let mut imgDyn = image::DynamicImage::new_rgba8(width, height);
        let length = width * height;
        let yLen = length / width;
        let xLen = length / height;

        for y in 0..yLen {
            for x in 0..xLen {
                let slicePosition = (y as usize * width as usize + x as usize) * 4;
                let red = imgData.get(slicePosition + 0).unwrap();
                let green = imgData.get(slicePosition + 1).unwrap();
                let blue = imgData.get(slicePosition + 2).unwrap();
                let alpha = imgData.get(slicePosition + 3).unwrap();

                let pixel = image::Rgba([*red, *green, *blue, *alpha]);
                imgDyn.put_pixel(x, y, pixel);
            }
        }

        // let imgDyn = imgDyn.blur(82.2);
        // let imgDyn = imgDyn.brighten(42);

        let x = imgDyn.width() - self.qr.width();
        let y = imgDyn.height() - self.qr.height();

        image::imageops::overlay(&mut imgDyn, &self.qr, x, y);

        let output: &[u8] = &*imgDyn.to_rgba8().to_vec();

        let output =
            web_sys::ImageData::new_with_u8_clamped_array_and_sh(Clamped(&output), width, height)
                .unwrap();

        return output;
    }
}

#[wasm_bindgen]
pub fn get_canvas_width(canvas: web_sys::HtmlCanvasElement) -> u32 {
    canvas.width()
}

#[wasm_bindgen]
pub fn wasm_memory() -> JsValue {
    wasm_bindgen::memory()
}

#[wasm_bindgen]
pub fn greet() -> String {
    let msg = "Hello World24";
    return String::from(msg);
}

#[wasm_bindgen]
pub fn get_grayscale_image(_array: &[u8]) -> Vec<u8> {
    // let img = load_image_from_array(_array);
    // let img = img.grayscale();
    // let a = img.as_bytes();
    // return Vec::from(a);

    return (*_array).to_vec();
}
