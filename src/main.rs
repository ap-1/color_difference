use clap::Parser;

mod color;
mod error;

use color::Rgb;
use error::{INVALID_COLOR_FILE, INVALID_IMAGE_FILE, UNPARSEBALE_COLOR};

use deltae::Delta;
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

    /// Calculate squared RGB distances as well
    #[clap(short, long)]
    squared_dist: bool,
}

fn get_colors(color_file: &Option<PathBuf>) -> Vec<Rgb> {
    io::BufReader::new(
        fs::File::open(
            color_file
                .as_ref()
                .unwrap_or_else(|| panic!("{}", INVALID_COLOR_FILE)),
        )
        .unwrap_or_else(|_| panic!("{}", INVALID_COLOR_FILE)),
    )
    .lines()
    .map(|line| Rgb::from(line.unwrap_or_else(|_| panic!("{}", UNPARSEBALE_COLOR))))
    .collect()
}

fn get_image_color(image_file: &Option<PathBuf>) -> Rgb {
    let img = image::open(
        image_file
            .as_ref()
            .unwrap_or_else(|| panic!("{}", INVALID_IMAGE_FILE)),
    )
    .unwrap_or_else(|_| panic!("{}", INVALID_IMAGE_FILE))
    .into_rgb8();

    let (width, height) = img.dimensions();

    Rgb::from(img.get_pixel(width / 2, height / 2))
}

fn main() -> error::Result<()> {
    let args = Args::parse();

    let image_rgb = get_image_color(&args.image_file);
    let image_lab = image_rgb.to_lab();

    for color in get_colors(&args.color_file) {
        print!("{}", image_lab.delta(color.to_lab(), deltae::DE2000));
        println!(
            "{}",
            if args.squared_dist {
                format!("; {}", image_rgb.squared_distance(color))
            } else {
                String::new()
            }
        );
    }

    Ok(())
}
