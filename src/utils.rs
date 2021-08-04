use std::path::PathBuf;
use structopt::StructOpt;
use image::{self, DynamicImage, GenericImageView, imageops::FilterType};
use anyhow::{bail, Result, anyhow};
use rayon::prelude::*;
use std::fs::create_dir_all;

/// Simple image resizer built for the web
#[derive(StructOpt, Debug)]
#[structopt()]
pub struct Args {
    /// File name of image
    #[structopt(short, long)]
    file_name: PathBuf,

    /// Base width of image
    #[structopt(short, long)]
    base_width: u32,

    /// Creates a folder to store the resized images in the current working directory
    #[structopt(long)]
    pub folder: bool,
}

pub struct ImageData {
    image: DynamicImage,
    scaling: usize,
}

pub struct WidthData {
    width: u32,
    scaling: usize,
}

impl Args {
    pub fn new() -> Self {
        Self::from_args()
    }

    pub fn get_image(&self) -> Result<DynamicImage> {
        let img = image::open(&self.file_name)?;
        Ok(img)
    }

    pub fn get_resized_widths(&self, width: u32) -> Vec<WidthData> {
        let mut width_data_vec = Vec::new();
        let mut width_buffer = self.base_width;
        let mut scaling = 0;
        while width_buffer <= width {
            scaling += 1;
            let width_data = WidthData {
                width: width_buffer,
                scaling
            };
            width_data_vec.push(width_data);
            width_buffer += self.base_width;
        }
        width_data_vec
    }

    pub fn save_images(&self, image_data: Vec<ImageData>) -> Result<()> {
        let file_stem = self.file_name.file_stem()
            .ok_or_else(|| anyhow!("Failed to extract file stem of `file_name`"))?.to_str()
            .ok_or_else(|| anyhow!("The file stem of `file_name` is invalid Unicode"))?;
        let file_stem = match file_stem.strip_suffix("-full") {
            Some(x) => x,
            None => file_stem
        };
        let file_extension = self.file_name.extension()
            .ok_or_else(|| anyhow!("Failed to extract extension of `file_name`"))?;
        let mut path = PathBuf::new();
        if self.folder {
            create_dir_all("resized")?;
            path.push("resized");
        }
        for image in image_data {
            let mut path = path.clone();
            let file_name = if image.scaling == 1 {
                file_stem.to_string()
            } else {
                format!("{}@{}x", file_stem, image.scaling)
            };
            path.push(file_name);
            path.set_extension(file_extension);
            image.image.save(path)?;
        }
        Ok(())
    }

    pub fn get_image_width(&self, image: &DynamicImage) -> Result<u32> {
        let width = image.dimensions().0;
        if self.base_width > width {
            bail!("The base width cannot be more than the original width of the image")
        }
        Ok(width)
    }
}

pub fn get_resized_images(image: &DynamicImage, width_data: Vec<WidthData>) -> Vec<ImageData> {
    width_data.into_par_iter().map(|data| {
        ImageData {
            image: image.resize(data.width, u32::MAX, FilterType::Gaussian),
            scaling: data.scaling
        }
    }).collect()
}