use crate::color::Color;
use crate::utils::u8_to_u4;
use crate::ImageType;
use anyhow::Result;
use byteorder::{BigEndian, WriteBytesExt};
use png::{BitDepth, ColorType};
use std::io::{Read, Write};

pub struct PNGImage {
    data: Vec<u8>,
    color_type: ColorType,
    bit_depth: BitDepth,
    width: u32,
    height: u32,
}

impl PNGImage {
    pub fn read<R: Read>(r: R) -> Result<Self> {
        let decoder = png::Decoder::new(r);
        let mut reader = decoder.read_info()?;
        // Allocate the output buffer.
        let mut buf = vec![0; reader.output_buffer_size()];
        // Read the next frame. An APNG might contain multiple frames.
        let info = reader.next_frame(&mut buf)?;

        // Grab the bytes of the image.
        let input_bytes = &buf[..info.buffer_size()];

        Ok(PNGImage {
            data: input_bytes.to_vec(),
            color_type: info.color_type,
            bit_depth: info.bit_depth,
            width: info.width,
            height: info.height,
        })
    }

    pub fn flip(&self, flip_x: bool, flip_y: bool) -> PNGImage {
        let mut flipped_bytes = vec![0; self.data.len()];

        match (flip_x, flip_y) {
            (true, true) => {
                for y in 0..self.height {
                    for x in 0..self.width {
                        let old_index = (y * self.width + x) as usize * self.color_type.samples();
                        let new_index = ((self.height - y - 1) * self.width + (self.width - x - 1))
                            as usize
                            * self.color_type.samples();
                        flipped_bytes[new_index..new_index + self.color_type.samples()]
                            .copy_from_slice(
                                &self.data[old_index..old_index + self.color_type.samples()],
                            );
                    }
                }
            }
            (true, false) => {
                for y in 0..self.height {
                    for x in 0..self.width {
                        let old_index = (y * self.width + x) as usize * self.color_type.samples();
                        let new_index = (y * self.width + (self.width - x - 1)) as usize
                            * self.color_type.samples();
                        flipped_bytes[new_index..new_index + self.color_type.samples()]
                            .copy_from_slice(
                                &self.data[old_index..old_index + self.color_type.samples()],
                            );
                    }
                }
            }
            (false, true) => {
                for y in 0..self.height {
                    for x in 0..self.width {
                        let old_index = (y * self.width + x) as usize * self.color_type.samples();
                        let new_index = ((self.height - y - 1) * self.width + x) as usize
                            * self.color_type.samples();
                        flipped_bytes[new_index..new_index + self.color_type.samples()]
                            .copy_from_slice(
                                &self.data[old_index..old_index + self.color_type.samples()],
                            );
                    }
                }
            }
            (false, false) => {
                flipped_bytes.copy_from_slice(&self.data);
            }
        }

        // return self with the new flipped bytes
        PNGImage {
            data: flipped_bytes,
            color_type: self.color_type,
            bit_depth: self.bit_depth,
            width: self.width,
            height: self.height,
        }
    }

    /// Writes the image as a PNG to the given writer.
    /// This is useful for when you need to flip an existing image.
    pub fn as_png<W: Write>(&self, writer: &mut W) -> Result<()> {
        let mut encoder = png::Encoder::new(writer, self.width, self.height);
        encoder.set_color(self.color_type);
        encoder.set_depth(self.bit_depth);
        let mut writer = encoder.write_header()?;
        writer.write_image_data(&self.data)?;
        Ok(())
    }

    pub fn as_native<W: Write>(&self, writer: &mut W, image_type: ImageType) -> Result<()> {
        match image_type {
            ImageType::I4 => self.as_i4(writer),
            ImageType::I8 => self.as_i8(writer),
            ImageType::Ia4 => self.as_ia4(writer),
            ImageType::Ia8 => self.as_ia8(writer),
            ImageType::Ia16 => self.as_ia16(writer),
            ImageType::Ci4 => self.as_ci4(writer),
            ImageType::Ci8 => self.as_ci8(writer),
            ImageType::Rgba32 => self.as_rgba32(writer),
            ImageType::Rgba16 => self.as_rgba16(writer),
        }
    }

    pub fn as_ci8<W: Write>(&self, writer: &mut W) -> Result<()> {
        assert_eq!(self.bit_depth, BitDepth::Eight);
        assert_eq!(self.color_type, ColorType::Indexed);
        writer.write_all(&self.data)?;

        Ok(())
    }

    pub fn as_ci4<W: Write>(&self, writer: &mut W) -> Result<()> {
        assert_eq!(self.color_type, ColorType::Indexed);

        match self.bit_depth {
            BitDepth::Four => writer.write_all(&self.data)?,
            BitDepth::Eight => self
                .data
                .chunks_exact(2)
                .for_each(|chunk| writer.write_u8(chunk[0] << 4 | chunk[1]).unwrap()),
            _ => panic!("unsupported bit depth: {:?}", self.bit_depth),
        }

        Ok(())
    }

    pub fn as_i4<W: Write>(&self, writer: &mut W) -> Result<()> {
        match (self.color_type, self.bit_depth) {
            (ColorType::Grayscale, BitDepth::Four) => writer.write_all(&self.data)?,
            (ColorType::Grayscale, BitDepth::Eight) => self
                .data
                .chunks_exact(2)
                .for_each(|chunk| writer.write_u8(chunk[0] << 4 | u8_to_u4(chunk[1])).unwrap()),
            (ColorType::Rgba, BitDepth::Eight) => self.data.chunks_exact(8).for_each(|chunk| {
                let c1 = Color::RGBA(chunk[0], chunk[1], chunk[2], chunk[3]);
                let i1 = c1.rgb_to_intensity();
                let c2 = Color::RGBA(chunk[4], chunk[5], chunk[6], chunk[7]);
                let i2 = c2.rgb_to_intensity();

                writer.write_u8(u8_to_u4(i1) << 4 | u8_to_u4(i2)).unwrap();
            }),
            (ColorType::Rgb, BitDepth::Eight) => self.data.chunks_exact(6).for_each(|chunk| {
                let c1 = Color::RGB(chunk[0], chunk[1], chunk[2]);
                let i1 = c1.rgb_to_intensity();
                let c2 = Color::RGB(chunk[3], chunk[4], chunk[5]);
                let i2 = c2.rgb_to_intensity();

                writer.write_u8(u8_to_u4(i1) << 4 | u8_to_u4(i2)).unwrap();
            }),
            p => panic!("unsupported format {:?}", p),
        }

        Ok(())
    }

    pub fn as_i8<W: Write>(&self, writer: &mut W) -> Result<()> {
        match (self.color_type, self.bit_depth) {
            (ColorType::Grayscale, BitDepth::Eight) => writer.write_all(&self.data)?,
            (ColorType::Grayscale, BitDepth::Four) => self
                .data
                .chunks_exact(2)
                .for_each(|chunk| writer.write_u8(chunk[0] << 4 | chunk[1]).unwrap()),
            (ColorType::Rgba, BitDepth::Eight) => self.data.chunks_exact(4).for_each(|chunk| {
                let c = Color::RGBA(chunk[0], chunk[1], chunk[2], chunk[3]);
                writer.write_u8(c.rgb_to_intensity()).unwrap();
            }),
            (ColorType::Rgb, BitDepth::Eight) => self.data.chunks_exact(3).for_each(|chunk| {
                let c = Color::RGB(chunk[0], chunk[1], chunk[2]);
                writer.write_u8(c.rgb_to_intensity()).unwrap();
            }),
            p => panic!("unsupported format {:?}", p),
        }

        Ok(())
    }

    pub fn as_ia4<W: Write>(&self, writer: &mut W) -> Result<()> {
        match (self.color_type, self.bit_depth) {
            (ColorType::GrayscaleAlpha, BitDepth::Eight) => {
                self.data.chunks_exact(4).for_each(|chunk| {
                    let intensity = (chunk[0] >> 5) << 1;
                    let alpha = (chunk[1] > 127) as u8;
                    let high = intensity | alpha;

                    let intensity = (chunk[2] >> 5) << 1;
                    let alpha = (chunk[3] > 127) as u8;
                    let low = intensity | alpha;

                    writer.write_u8(high << 4 | (low & 0xF)).unwrap();
                })
            }
            (ColorType::Rgba, BitDepth::Eight) => self.data.chunks_exact(8).for_each(|chunk| {
                let c1 = Color::RGBA(chunk[0], chunk[1], chunk[2], chunk[3]);
                let intensity1 = (c1.rgb_to_intensity() >> 5) << 1;
                let alpha1 = (c1.a > 127) as u8;

                let c2 = Color::RGBA(chunk[4], chunk[5], chunk[6], chunk[7]);
                let intensity2 = (c2.rgb_to_intensity() >> 5) << 1;
                let alpha2 = (c2.a > 127) as u8;

                let high = intensity1 | alpha1;
                let low = intensity2 | alpha2;
                writer.write_u8(high << 4 | (low & 0xF)).unwrap();
            }),
            p => panic!("unsupported format {:?}", p),
        }

        Ok(())
    }

    pub fn as_ia8<W: Write>(&self, writer: &mut W) -> Result<()> {
        match (self.color_type, self.bit_depth) {
            (ColorType::GrayscaleAlpha, BitDepth::Eight) => self
                .data
                .chunks_exact(2)
                .for_each(|chunk| writer.write_u8(chunk[0] << 4 | (chunk[1] & 0x0F)).unwrap()),
            (ColorType::Rgba, BitDepth::Eight) => self.data.chunks_exact(4).for_each(|chunk| {
                let c = Color::RGBA(chunk[0], chunk[1], chunk[2], chunk[3]);
                let i = (c.rgb_to_intensity() >> 4) & 0xF;
                let a = (c.a >> 4) & 0xF;

                writer.write_u8(i << 4 | a).unwrap();
            }),
            p => panic!("unsupported format {:?}", p),
        }

        Ok(())
    }

    pub fn as_ia16<W: Write>(&self, writer: &mut W) -> Result<()> {
        match (self.color_type, self.bit_depth) {
            (ColorType::GrayscaleAlpha, BitDepth::Eight) => writer.write_all(&self.data)?,
            (ColorType::Rgba, BitDepth::Eight) => self.data.chunks_exact(4).for_each(|chunk| {
                let c = Color::RGBA(chunk[0], chunk[1], chunk[2], chunk[3]);
                let i = c.rgb_to_intensity();
                let a = c.a;

                writer.write_u8(i).unwrap();
                writer.write_u8(a).unwrap();
            }),
            p => panic!("unsupported format {:?}", p),
        }

        Ok(())
    }

    pub fn as_rgba16<W: Write>(&self, writer: &mut W) -> Result<()> {
        match (self.color_type, self.bit_depth) {
            (ColorType::Rgba, BitDepth::Eight) => self.data.chunks_exact(4).for_each(|chunk| {
                let color = Color::RGBA(chunk[0], chunk[1], chunk[2], chunk[3]);
                writer.write_u16::<BigEndian>(color.to_u16()).unwrap();
            }),
            p => panic!("unsupported format {:?}", p),
        }

        Ok(())
    }

    pub fn as_rgba32<W: Write>(&self, writer: &mut W) -> Result<()> {
        match (self.color_type, self.bit_depth) {
            (ColorType::Rgba, BitDepth::Eight) => writer.write_all(&self.data)?,
            p => panic!("unsupported format {:?}", p),
        }

        Ok(())
    }
}

pub fn create_palette_from_png<R: Read, W: Write>(r: R, writer: &mut W) -> Result<()> {
    let decoder = png::Decoder::new(r);
    let reader = decoder.read_info()?;
    let info = reader.info();
    let rgb_data = info.palette.as_ref().expect("given PNG has no palette");
    let alpha_data = info.trns.as_ref();

    match alpha_data {
        Some(alpha_data) => {
            rgb_data
                .chunks_exact(3)
                .zip(alpha_data.iter())
                .for_each(|(rgb, &alpha)| {
                    let color = Color::RGBA(rgb[0], rgb[1], rgb[2], alpha);
                    writer.write_u16::<BigEndian>(color.to_u16()).unwrap();
                })
        }

        // If there's no alpha channel, assume everything is opaque
        None => rgb_data.chunks_exact(3).for_each(|rgb| {
            let color = Color::RGB(rgb[0], rgb[1], rgb[2]);
            writer.write_u16::<BigEndian>(color.to_u16()).unwrap();
        }),
    }

    Ok(())
}
