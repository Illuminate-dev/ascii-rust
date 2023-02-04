pub mod commands;
pub mod args;

use image::{io::Reader as ImageReader, GenericImageView, Rgba};
use std::{cmp::{min,max}, str::FromStr};

pub enum AsciiError {
    InvalidCharType
}

const ALPHABET: &str = "`^\",:;Il!i~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum CharType {
    Average,
    Luminosity,
    Brightness
}

impl FromStr for CharType {
    type Err = AsciiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(CharType::Average)
    }
}

impl std::fmt::Display for CharType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

pub fn image_to_string(filename: &str, width: u32, height: u32, method: CharType) -> String {
    let mut output = String::new();

    let image = ImageReader::open(filename)
        .unwrap()
        .decode()
        .unwrap();

    let image = image.resize(width, height, image::imageops::FilterType::Lanczos3);

    for y in 0..image.height() {
        for x in 0..image.width() {
            let pixel = image.get_pixel(x, y);
            let char = calculate_char(&pixel, &method); 
            output.push_str(&format!("{char}{char}"));
        }
        output.push('\n');
    }

    output


}

pub fn calculate_char(pixel: &Rgba<u8>, method: &CharType) -> &'static str {

    let (red, green, blue) = (pixel.0[0], pixel.0[1], pixel.0[2]);

    let number = match method {
        CharType::Average => (red as f64 + green as f64 + blue as f64) / 3.0,
        CharType::Brightness => (min(red, min(green, blue)) as u16
        + max(red, max(green, blue)) as u16) as f64
        / 2.0,
        CharType::Luminosity => (0.21 * red as f64) + (0.72 * green as f64) + (0.07* blue as f64),
    };

    let index = ((number as f64 / 255.0) * 65.0) as usize;
    &ALPHABET[index - 1..index]
}

