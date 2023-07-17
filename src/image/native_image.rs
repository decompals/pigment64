use crate::color::Color;
use crate::ImageType;
use anyhow::Result;
use byteorder::{BigEndian, ReadBytesExt};
use std::io::{Read, Write};

/// reads an rgba color from a buffer starting at the given offset
fn get_tlut_color(tlut_color_table: Option<&[u8]>, index: u8) -> [u8; 4] {
    if let Some(tlut_color_table) = tlut_color_table {
        let r = tlut_color_table[(index * 4) as usize];
        let g = tlut_color_table[((index * 4) + 1) as usize];
        let b = tlut_color_table[((index * 4) + 2) as usize];
        let a = tlut_color_table[((index * 4) + 3) as usize];
        [r, g, b, a]
    } else {
        [index, index, index, 0xFF]
    }
}

pub struct NativeImage {
    pub format: ImageType,
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

impl NativeImage {
    pub fn read<R: Read>(
        mut reader: R,
        format: ImageType,
        width: u32,
        height: u32,
    ) -> Result<Self> {
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;

        Ok(Self {
            format,
            width,
            height,
            data,
        })
    }

    /// Decodes the image into RGBA32 format.
    pub fn decode<W: Write>(&self, writer: &mut W, tlut_color_table: Option<&[u8]>) -> Result<()> {
        let mut cursor = std::io::Cursor::new(&self.data);

        match self.format {
            ImageType::I4 => {
                for _y in 0..self.height {
                    for _x in (0..self.width).step_by(2) {
                        let byte = cursor.read_u8()?;

                        let intensity = byte & 0xF0;
                        writer.write_all(&[intensity, intensity, intensity, 0xFF])?;

                        let intensity = (byte & 0x0F) << 4;
                        writer.write_all(&[intensity, intensity, intensity, 0xFF])?;
                    }
                }
            }
            ImageType::I8 => {
                for _y in 0..self.height {
                    for _x in 0..self.width {
                        let intensity = cursor.read_u8()?;
                        writer.write_all(&[intensity, intensity, intensity, intensity])?;
                    }
                }
            }
            ImageType::Ia4 => {
                for _y in 0..self.height {
                    for _x in (0..self.width).step_by(2) {
                        let byte = cursor.read_u8()?;

                        let source = (byte & 0xF0) >> 4;
                        let intensity = ((source & 0x0E) >> 1) * 32;
                        let alpha = (source & 0x01) * 255;
                        writer.write_all(&[intensity, intensity, intensity, alpha])?;

                        let source = byte & 0x0F;
                        let intensity = ((source & 0x0E) >> 1) * 32;
                        let alpha = (source & 0x01) * 255;
                        writer.write_all(&[intensity, intensity, intensity, alpha])?;
                    }
                }
            }
            ImageType::Ia8 => {
                for _y in 0..self.height {
                    for _x in 0..self.width {
                        let byte = cursor.read_u8()?;

                        let intensity = byte & 0xF0;
                        let alpha = (byte & 0x0F) << 4;

                        writer.write_all(&[intensity, intensity, intensity, alpha])?;
                    }
                }
            }
            ImageType::Ia16 => {
                for _y in 0..self.height {
                    for _x in 0..self.width {
                        let intensity = cursor.read_u8()?;
                        let alpha = cursor.read_u8()?;

                        writer.write_all(&[intensity, intensity, intensity, alpha])?;
                    }
                }
            }
            ImageType::Ci4 => {
                assert!(tlut_color_table.is_some());

                for _y in 0..self.height {
                    for _x in (0..self.width).step_by(2) {
                        let byte = cursor.read_u8()?;

                        let index = (byte >> 4) & 0x0F;
                        writer.write_all(&get_tlut_color(tlut_color_table, index))?;

                        let index = byte & 0x0F;
                        writer.write_all(&get_tlut_color(tlut_color_table, index))?;
                    }
                }
            }
            ImageType::Ci8 => {
                assert!(tlut_color_table.is_some());

                for _y in 0..self.height {
                    for _x in 0..self.width {
                        let index = cursor.read_u8()?;
                        writer.write_all(&get_tlut_color(tlut_color_table, index))?;
                    }
                }
            }
            ImageType::Rgba16 => {
                for _y in 0..self.height {
                    for _x in 0..self.width {
                        let pixel = cursor.read_u16::<BigEndian>()?;
                        let color = Color::from_u16(pixel);
                        writer.write_all(&color.rgba16())?;
                    }
                }
            }
            ImageType::Rgba32 => {
                for _y in 0..self.height {
                    for _x in 0..self.width {
                        let pixel = cursor.read_u32::<BigEndian>()?;
                        let color = Color::from_u32(pixel);
                        writer.write_all(&[color.r, color.g, color.b, color.a])?;
                    }
                }
            }
        }

        Ok(())
    }
}
