use crate::{image_to_string, CharType};

use super::args::Args;
use clap::Parser;

pub fn run() {
    let args = Args::parse();

    let filename = args.filepath;

    let width = args.width.unwrap_or(200);
    let height = args.height.unwrap_or(200);

    let method = args.char_type.unwrap_or(CharType::Average);

    let ascii_image = image_to_string(filename.to_str().unwrap(), width, height, method);

    print!("{ascii_image}");
}
