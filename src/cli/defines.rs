use crate::cli::binary::CArrayWidth;
use clap::ValueEnum;
use pigment64::{ImageSize, ImageType};

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

    pub fn as_native(&self) -> Option<ImageType> {
        match self {
            BinaryFormat::Ci4 => Some(ImageType::Ci4),
            BinaryFormat::Ci8 => Some(ImageType::Ci8),
            BinaryFormat::I1 => Some(ImageType::I1),
            BinaryFormat::I4 => Some(ImageType::I4),
            BinaryFormat::I8 => Some(ImageType::I8),
            BinaryFormat::Ia4 => Some(ImageType::Ia4),
            BinaryFormat::Ia8 => Some(ImageType::Ia8),
            BinaryFormat::Ia16 => Some(ImageType::Ia16),
            BinaryFormat::Rgba16 => Some(ImageType::Rgba16),
            BinaryFormat::Rgba32 => Some(ImageType::Rgba32),
            BinaryFormat::Palette => None,
        }
    }

    pub fn get_size(&self) -> Option<ImageSize> {
        match self {
            BinaryFormat::Ci4 => Some(ImageSize::Bits4),
            BinaryFormat::Ci8 => Some(ImageSize::Bits8),
            BinaryFormat::I1 => Some(ImageSize::Bits1),
            BinaryFormat::I4 => Some(ImageSize::Bits4),
            BinaryFormat::I8 => Some(ImageSize::Bits8),
            BinaryFormat::Ia4 => Some(ImageSize::Bits4),
            BinaryFormat::Ia8 => Some(ImageSize::Bits8),
            BinaryFormat::Ia16 => Some(ImageSize::Bits16),
            BinaryFormat::Rgba16 => Some(ImageSize::Bits16),
            BinaryFormat::Rgba32 => Some(ImageSize::Bits32),
            BinaryFormat::Palette => None,
        }
    }
}
