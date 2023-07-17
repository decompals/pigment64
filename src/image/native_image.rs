use crate::color::Color;
use crate::{ImageSize, ImageType, TextureLUT};
use anyhow::Result;
use byteorder::{BigEndian, ReadBytesExt};
use std::io::{Cursor, Read, Write};

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

    /// Decodes the image into RGBA8 format and writes it image bytes to the given writer.
    pub fn decode<W: Write>(&self, writer: &mut W, tlut_color_table: Option<&[u8]>) -> Result<()> {
        let mut cursor = Cursor::new(&self.data);

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
                        writer.write_all(&[intensity, intensity, intensity, 0xFF])?;
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
                        writer.write_all(&get_tlut_color_at_index(tlut_color_table, index))?;

                        let index = byte & 0x0F;
                        writer.write_all(&get_tlut_color_at_index(tlut_color_table, index))?;
                    }
                }
            }
            ImageType::Ci8 => {
                assert!(tlut_color_table.is_some());

                for _y in 0..self.height {
                    for _x in 0..self.width {
                        let index = cursor.read_u8()?;
                        writer.write_all(&get_tlut_color_at_index(tlut_color_table, index))?;
                    }
                }
            }
            ImageType::Rgba16 => {
                for _y in 0..self.height {
                    for _x in 0..self.width {
                        let pixel = cursor.read_u16::<BigEndian>()?;
                        let color = Color::from_u16(pixel);
                        writer.write_all(&[color.r, color.g, color.b, color.a])?;
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

    /// Decodes the image into RGBA8 and writes it as PNG to the given writer.
    /// Exception is CI4 and CI8, which get written as an indexed PNG.
    pub fn as_png<W: Write>(&self, writer: &mut W, tlut_color_table: Option<&[u8]>) -> Result<()> {
        let mut data: Vec<u8> = vec![];
        let mut encoder = png::Encoder::new(writer, self.width, self.height);

        match self.format {
            ImageType::I4 => {
                self.decode(&mut data, None)?;
            }
            ImageType::I8 => {
                self.decode(&mut data, None)?;
            }
            ImageType::Ia4 => {
                self.decode(&mut data, None)?;
            }
            ImageType::Ia8 => {
                self.decode(&mut data, None)?;
            }
            ImageType::Ia16 => {
                self.decode(&mut data, None)?;
            }
            ImageType::Ci4 => {
                assert!(tlut_color_table.is_some());
                let mut cursor = Cursor::new(&self.data);
                let mut data: Vec<u8> = vec![];

                for _y in 0..self.height {
                    for _x in (0..self.width).step_by(2) {
                        let byte = cursor.read_u8()?;
                        data.push(byte);
                    }
                }

                encoder.set_color(png::ColorType::Indexed);
                encoder.set_depth(png::BitDepth::Four);

                let (palette_data, trans_data) =
                    split_color_table_for_png(tlut_color_table.unwrap());

                encoder.set_palette(palette_data);
                encoder.set_trns(trans_data);

                let mut writer = encoder.write_header()?;
                writer.write_image_data(&data)?;

                return Ok(());
            }
            ImageType::Ci8 => {
                assert!(tlut_color_table.is_some());
                let mut cursor = Cursor::new(&self.data);
                let mut data: Vec<u8> = vec![];

                for _y in 0..self.height {
                    for _x in 0..self.width {
                        let index = cursor.read_u8()?;
                        data.push(index);
                    }
                }

                encoder.set_color(png::ColorType::Indexed);
                encoder.set_depth(png::BitDepth::Eight);

                let (palette_data, trans_data) =
                    split_color_table_for_png(tlut_color_table.unwrap());

                encoder.set_palette(palette_data);
                encoder.set_trns(trans_data);

                let mut writer = encoder.write_header()?;
                writer.write_image_data(&data)?;

                return Ok(());
            }
            ImageType::Rgba16 => {
                self.decode(&mut data, None)?;
            }
            ImageType::Rgba32 => {
                self.decode(&mut data, None)?;
            }
        }

        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header()?;
        writer.write_image_data(&data)?;

        Ok(())
    }
}

/// Parses a tlut into a RGBA8 color table
pub fn parse_tlut(bytes: &[u8], size: ImageSize, mode: TextureLUT) -> Result<Vec<u8>> {
    assert_eq!(
        mode,
        TextureLUT::Rgba16,
        "Only RGBA16 TLUTs are supported at the moment"
    );

    let mut output: Vec<u8> = vec![];
    let cursor = &mut Cursor::new(bytes);
    for _i in 0..(size.get_tlut_size()) {
        let pixel = cursor.read_u16::<BigEndian>()?;
        let color = Color::from_u16(pixel);
        output.write_all(&[color.r, color.g, color.b, color.a])?;
    }

    Ok(output)
}

/// Reads an rgba color from a buffer starting at the given offset
fn get_tlut_color_at_index(tlut_color_table: Option<&[u8]>, index: u8) -> [u8; 4] {
    assert!(tlut_color_table.is_some());

    if let Some(tlut_color_table) = tlut_color_table {
        let r = tlut_color_table[(index * 4) as usize];
        let g = tlut_color_table[((index * 4) + 1) as usize];
        let b = tlut_color_table[((index * 4) + 2) as usize];
        let a = tlut_color_table[((index * 4) + 3) as usize];

        return [r, g, b, a];
    }

    unreachable!()
}

/// Splits a color table into a palette and a transparency table for png encoding
fn split_color_table_for_png(table: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let palette_data: Vec<u8> = table
        .chunks(4)
        .flat_map(|chunk| chunk[..3].to_vec())
        .collect();

    let trans_data: Vec<u8> = table.chunks(4).map(|chunk| chunk[3]).collect();

    (palette_data, trans_data)
}
