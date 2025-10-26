use wasm_bindgen::prelude::*;
use web_sys::console::log_1 as log;
use base64::{prelude::BASE64_STANDARD, Engine };
use image::load_from_memory;
use std::io::Cursor;
use image::ImageFormat::Png;

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String{
  log(&"Grayscale called".into());

  let base64_to_vector = BASE64_STANDARD.decode(encoded_file).unwrap();
  log(&"Image decoded".into());

  let mut img = load_from_memory(&base64_to_vector).unwrap();
  log(&"Image loaded".into());

  img = img.grayscale();
  log(&"Grayscale effect applied".into());

  let mut buffer = vec![];
  img.write_to(&mut Cursor::new(&mut buffer), Png).unwrap();
  log(&"New image written".into());

  let encoded_img = BASE64_STANDARD.encode(&buffer);
  let data_url = format!(
    "data:image/png;base64,{}",
    encoded_img
  );

  data_url
}