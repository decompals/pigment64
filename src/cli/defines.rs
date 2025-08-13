use crate::cli::binary::CArrayWidth;
use clap::ValueEnum;
use pigment64::{Error, ImageSize, ImageType};

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
pub enum BinaryFormat {
    Ci4,
    Ci8,
    I1,
    I4,
    I8,
    Ia4,
    Ia8,
    Ia16,
    Rgba16,
    Rgba32,
    Palette,
}

impl BinaryFormat {
    pub fn get_width(&self) -> CArrayWidth {
        match self {
            BinaryFormat::Ci4 => CArrayWidth::U8,
            BinaryFormat::Ci8 => CArrayWidth::U8,
            BinaryFormat::I1 => CArrayWidth::U8,
            BinaryFormat::I4 => CArrayWidth::U8,
            BinaryFormat::I8 => CArrayWidth::U8,
            BinaryFormat::Ia4 => CArrayWidth::U8,
            BinaryFormat::Ia8 => CArrayWidth::U8,
            BinaryFormat::Ia16 => CArrayWidth::U16,
            BinaryFormat::Rgba16 => CArrayWidth::U16,
            BinaryFormat::Rgba32 => CArrayWidth::U32,
            BinaryFormat::Palette => CArrayWidth::U16,
        }
    }

    pub fn as_native(&self) -> Result<ImageType, Error> {
        match self {
            BinaryFormat::Ci4 => Ok(ImageType::Ci4),
            BinaryFormat::Ci8 => Ok(ImageType::Ci8),
            BinaryFormat::I1 => Ok(ImageType::I1),
            BinaryFormat::I4 => Ok(ImageType::I4),
            BinaryFormat::I8 => Ok(ImageType::I8),
            BinaryFormat::Ia4 => Ok(ImageType::Ia4),
            BinaryFormat::Ia8 => Ok(ImageType::Ia8),
            BinaryFormat::Ia16 => Ok(ImageType::Ia16),
            BinaryFormat::Rgba16 => Ok(ImageType::Rgba16),
            BinaryFormat::Rgba32 => Ok(ImageType::Rgba32),
            BinaryFormat::Palette => Err(Error::PaletteConversionError),
        }
    }

    pub fn get_size(&self) -> Result<ImageSize, Error> {
        match self {
            BinaryFormat::Ci4 => Ok(ImageSize::Bits4),
            BinaryFormat::Ci8 => Ok(ImageSize::Bits8),
            BinaryFormat::I1 => Ok(ImageSize::Bits1),
            BinaryFormat::I4 => Ok(ImageSize::Bits4),
            BinaryFormat::I8 => Ok(ImageSize::Bits8),
            BinaryFormat::Ia4 => Ok(ImageSize::Bits4),
            BinaryFormat::Ia8 => Ok(ImageSize::Bits8),
            BinaryFormat::Ia16 => Ok(ImageSize::Bits16),
            BinaryFormat::Rgba16 => Ok(ImageSize::Bits16),
            BinaryFormat::Rgba32 => Ok(ImageSize::Bits32),
            BinaryFormat::Palette => Err(Error::PaletteConversionError),
        }
    }
}
