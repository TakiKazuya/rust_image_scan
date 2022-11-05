use std::env;
use opencv::core::Mat;
use opencv::text::{OEM_DEFAULT, PSM_AUTO};
use opencv::text::prelude::OCRTesseract;

pub fn get_words(src_image: &Mat) -> String {
    // OCR
    let mut ocr = <dyn OCRTesseract>::create("", "jpn", "", OEM_DEFAULT, PSM_AUTO)
        .unwrap_or_else(|code| panic!("{}", code));
    ocr.run(src_image, 0, 0).unwrap()
}