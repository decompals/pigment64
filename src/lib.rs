pub mod color;

pub mod image;
pub use image::png_image::PNGImage;
pub use image::png_image::create_palette_from_png;

mod utils;
