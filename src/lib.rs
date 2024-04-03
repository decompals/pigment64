use num_enum::TryFromPrimitive;

pub mod color;

pub mod image;
pub use image::native_image::NativeImage;
pub use image::png_image::create_palette_from_png;
pub use image::png_image::PNGImage;

mod utils;

use strum_macros::{EnumCount, EnumIter};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, TryFromPrimitive)]
#[repr(u8)]
pub enum ImageSize {
    Bits4 = 0,
    Bits8 = 1,
    Bits16 = 2,
    Bits32 = 3,
    Bits1 = 4,
    DD = 5,
}

impl ImageSize {
    /// Returns the size of the TLUT (Table Look-Up Table) based on the image size.
    ///
    /// # Returns
    ///
    /// The size of the TLUT as a `usize` value.
    ///
    /// # Panics
    ///
    /// This method will panic if the image size is invalid.
    pub fn get_tlut_size(&self) -> usize {
        match self {
            ImageSize::Bits1 => 0b10,
            ImageSize::Bits4 => 0x10,
            ImageSize::Bits8 => 0x100,
            ImageSize::Bits16 => 0x1000,
            ImageSize::Bits32 => 0x10000,
            _ => panic!("Invalid size: {:?}", self),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, TryFromPrimitive)]
#[repr(u8)]
pub enum ImageFormat {
    Rgba = 0,
    Yuv = 1,
    Ci = 2,
    Ia = 3,
    I = 4,
}

/// Represents the type of image.
///
/// This enum is used to specify the type of image, which determines the size and format of the
/// image data.
/// Each variant corresponds to a specific image type, such as indexed color (Ci), grayscale (I),
/// grayscale with alpha (Ia), or red-green-blue-alpha (RGBA).
///
#[derive(Copy, Clone, Debug, PartialEq, EnumCount, EnumIter, Eq, Hash, TryFromPrimitive)]
#[repr(u8)]
pub enum ImageType {
    I1,
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
    /// Returns the size of the image type.
    ///
    /// This function returns the size of the image type, which represents the number of bits used
    /// to store each pixel. The size is determined based on the image type variant.
    ///
    /// # Returns
    ///
    /// - `ImageSize` - The size of the image type.
    pub fn get_size(&self) -> ImageSize {
        match self {
            ImageType::Ci4 => ImageSize::Bits4,
            ImageType::Ci8 => ImageSize::Bits8,
            ImageType::I1 => ImageSize::Bits1,
            ImageType::I4 => ImageSize::Bits4,
            ImageType::I8 => ImageSize::Bits8,
            ImageType::Ia4 => ImageSize::Bits4,
            ImageType::Ia8 => ImageSize::Bits8,
            ImageType::Ia16 => ImageSize::Bits16,
            ImageType::Rgba16 => ImageSize::Bits16,
            ImageType::Rgba32 => ImageSize::Bits32,
        }
    }

    /// Returns the format of the image type.
    ///
    /// This method returns the format of the image type, which represents the color model used by
    /// the image. The format is determined based on the image type variant.
    ///
    /// # Returns
    ///
    /// - `ImageFormat` - The format of the image type.
    pub fn get_format(&self) -> ImageFormat {
        match self {
            ImageType::Ci4 => ImageFormat::Ci,
            ImageType::Ci8 => ImageFormat::Ci,
            ImageType::I1 => ImageFormat::I,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive)]
#[repr(u8)]
pub enum TextureLUT {
    None = 0,
    Rgba16 = 2,
    Ia16 = 3,
}
