use crate::cli::binary::CArrayWidth;
use clap::ValueEnum;
use pigment64::ImageSize::Bits4;
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

    pub fn as_native(&self) -> ImageType {
        match self {
            BinaryFormat::Ci4 => ImageType::Ci4,
            BinaryFormat::Ci8 => ImageType::Ci8,
            BinaryFormat::I1 => ImageType::I1,
            BinaryFormat::I4 => ImageType::I4,
            BinaryFormat::I8 => ImageType::I8,
            BinaryFormat::Ia4 => ImageType::Ia4,
            BinaryFormat::Ia8 => ImageType::Ia8,
            BinaryFormat::Ia16 => ImageType::Ia16,
            BinaryFormat::Rgba16 => ImageType::Rgba16,
            BinaryFormat::Rgba32 => ImageType::Rgba32,
            BinaryFormat::Palette => panic!("cannot convert palette to native format"),
        }
    }

    pub fn get_size(&self) -> ImageSize {
        match self {
            BinaryFormat::Ci4 => Bits4,
            BinaryFormat::Ci8 => ImageSize::Bits8,
            BinaryFormat::I1 => ImageSize::Bits1,
            BinaryFormat::I4 => ImageSize::Bits4,
            BinaryFormat::I8 => ImageSize::Bits8,
            BinaryFormat::Ia4 => ImageSize::Bits4,
            BinaryFormat::Ia8 => ImageSize::Bits8,
            BinaryFormat::Ia16 => ImageSize::Bits16,
            BinaryFormat::Rgba16 => ImageSize::Bits16,
            BinaryFormat::Rgba32 => ImageSize::Bits32,
            BinaryFormat::Palette => panic!("cannot convert palette to native format"),
        }
    }
}
