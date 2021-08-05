#![warn(clippy::pedantic)]

mod utils;

use anyhow::{Context, Result};
use std::io::{stdout, Write};
#[cfg(not(windows))]
use ansi_term::Colour::Green;

fn main() -> Result<()> {
    let img = utils::Args::new();
    let original_image = img.get_image().with_context(|| "Failed to open the image")?;
    let original_img_width = img.get_image_width(&original_image).with_context(|| "Failed to get the image width")?;
    let width_data = img.get_resized_widths(original_img_width);
    let image_data = utils::get_resized_images(original_image, width_data);
    img.save_images(image_data).with_context(|| "Failed to save the image")?;
    #[cfg(not(windows))]
    writeln!(stdout(), "{}", Green.paint("Resized images generated and saved"))?;
    #[cfg(windows)]
    writeln!(stdout(), "Resized images generated and saved")?;
    Ok(())
}
