pub mod color;
pub mod image;

pub use crate::image::native_image::NativeImage;
pub use crate::image::png_image::{PNGImage, create_palette_from_png};

// Imports from external crates
use num_enum::TryFromPrimitive;
use strum_macros::{EnumCount, EnumIter};
use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("Invalid image size for TLUT: {0:?}")]
    InvalidSizeForTlut(ImageSize),
    #[error("A TLUT color table is required for this image format")]
    MissingTlut,
    #[error("The specified TLUT mode is not supported: {0:?}")]
    UnsupportedTlutMode(TextureLUT),
    #[error("TLUT index is out of bounds")]
    TlutIndexOutOfBounds,
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("PNG encoding error: {0}")]
    PngEncoding(#[from] png::EncodingError),
    #[error("PNG decoding error: {0}")]
    PngDecoding(#[from] png::DecodingError),
    #[error("PNG is missing a palette, which is required for this operation")]
    MissingPngPalette,
    #[error(
        "Unsupported PNG format for conversion to {target_format:?}: color={color:?}, depth={depth:?}"
    )]
    UnsupportedPngConversion {
        color: png::ColorType,
        depth: png::BitDepth,
        target_format: ImageType,
    },
    #[error("Palette format cannot be converted to a native image format")]
    PaletteConversionError,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, TryFromPrimitive)]
#[repr(u8)]
pub enum ImageSize {
    Bits1 = 4,
    Bits4 = 0,
    Bits8 = 1,
    Bits16 = 2,
    Bits32 = 3,
    DD = 5,
}

impl ImageSize {
    /// Returns the size of the TLUT (Table Look-Up Table) based on the image size.
    ///
    /// # Returns
    ///
    /// An `Option` containing the size of the TLUT as a `usize` value, or `None`
    /// if the image size is not valid for a TLUT.
    pub fn get_tlut_size(&self) -> Option<usize> {
        match self {
            ImageSize::Bits1 => Some(0b10),
            ImageSize::Bits4 => Some(0x10),
            ImageSize::Bits8 => Some(0x100),
            ImageSize::Bits16 => Some(0x1000),
            ImageSize::Bits32 => Some(0x10000),
            _ => None,
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
