use super::CharType;
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    pub filepath: PathBuf,

    pub width: Option<u32>,
    pub height: Option<u32>,

    #[arg(short, long)]
    pub char_type: Option<CharType>,
}
