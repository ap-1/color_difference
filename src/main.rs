use clap::Parser;

mod color;
mod error;

use color::Rgb;
use error::{INVALID_COLOR_FILE, UNPARSEBALE_COLOR, INVALID_IMAGE_FILE};

use deltae::{Delta, LabValue};
use std::{
    fs,
    io::{self, BufRead},
    path::PathBuf,
};

#[derive(Parser, Debug)]
#[clap(about, author, version)]
struct Args {
    /// Path to file containing RGBs separated by line
    #[clap(short, long, parse(from_os_str))]
    color_file: Option<PathBuf>,

    /// Path to image to compare colors against
    #[clap(short, long, parse(from_os_str))]
    image_file: Option<PathBuf>,
}

fn get_lab_colors(color_file: Option<PathBuf>) -> Vec<LabValue> {
    io::BufReader::new(
        fs::File::open(
            color_file.unwrap_or_else(|| panic!("{}", INVALID_COLOR_FILE)),
        )
        .unwrap_or_else(|_| panic!("{}", INVALID_COLOR_FILE)),
    )
    .lines()
    .map(|line| {
        Rgb::from(line.unwrap_or_else(|_| panic!("{}", UNPARSEBALE_COLOR))).to_lab()
    })
    .collect()
}

fn get_image_color(image_file: Option<PathBuf>) -> LabValue {
    let img =
        image::open(image_file.unwrap_or_else(|| panic!("{}", INVALID_IMAGE_FILE)))
            .unwrap_or_else(|_| panic!("{}", INVALID_IMAGE_FILE))
            .into_rgb8();

    let (width, height) = img.dimensions();

    Rgb::from(img.get_pixel(width / 2, height / 2)).to_lab()
}

fn main() -> error::Result<()> {
    let args = Args::parse();

    let image_color = get_image_color(args.image_file);

    for lab in get_lab_colors(args.color_file) {
        println!("{}", image_color.delta(lab, deltae::DE2000));
    }

    Ok(())
}
