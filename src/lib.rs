pub mod color;

pub mod image;
pub use image::png_image::create_palette_from_png;
pub use image::png_image::PNGImage;

mod utils;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ImageSize {
    Bits4,
    Bits8 = 1,
    Bits16 = 2,
    Bits32 = 3,
    DD = 5,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ImageFormat {
    Rgba = 0,
    Yuv = 1,
    Ci = 2,
    Ia = 3,
    I = 4,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ImageType {
    I4,
    I8,
    Ia4,
    Ia8,
    Ia16,
    Ci4,
    Ci8,
    Rgba16,
    Rgba32,
}

impl ImageType {
    pub fn get_size(&self) -> ImageSize {
        match self {
            ImageType::Ci4 => ImageSize::Bits4,
            ImageType::Ci8 => ImageSize::Bits8,
            ImageType::I4 => ImageSize::Bits4,
            ImageType::I8 => ImageSize::Bits8,
            ImageType::Ia4 => ImageSize::Bits4,
            ImageType::Ia8 => ImageSize::Bits8,
            ImageType::Ia16 => ImageSize::Bits16,
            ImageType::Rgba16 => ImageSize::Bits16,
            ImageType::Rgba32 => ImageSize::Bits32,
        }
    }

    pub fn get_format(&self) -> ImageFormat {
        match self {
            ImageType::Ci4 => ImageFormat::Ci,
            ImageType::Ci8 => ImageFormat::Ci,
            ImageType::I4 => ImageFormat::I,
            ImageType::I8 => ImageFormat::I,
            ImageType::Ia4 => ImageFormat::Ia,
            ImageType::Ia8 => ImageFormat::Ia,
            ImageType::Ia16 => ImageFormat::Ia,
            ImageType::Rgba16 => ImageFormat::Rgba,
            ImageType::Rgba32 => ImageFormat::Rgba,
        }
    }
}

