use std::io::Read;
use png::{BitDepth, ColorType};
use crate::color::Color;
use crate::utils::u8_to_u4;

pub struct PNGImage {
    data: Vec<u8>,
    color_type: ColorType,
    bit_depth: BitDepth,
    width: u32,
    height: u32,
}

impl PNGImage {
    pub fn read_png<R: Read>(r: R) -> PNGImage {
        let decoder = png::Decoder::new(r);
        let mut reader = decoder.read_info().unwrap();
        // Allocate the output buffer.
        let mut buf = vec![0; reader.output_buffer_size()];
        // Read the next frame. An APNG might contain multiple frames.
        let info = reader.next_frame(&mut buf).unwrap();

        // Grab the bytes of the image.
        let input_bytes = &buf[..info.buffer_size()];

        PNGImage {
            data: input_bytes.to_vec(),
            color_type: info.color_type,
            bit_depth: info.bit_depth,
            width: info.width,
            height: info.height,
        }
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

    pub fn as_ci8(&self) -> Vec<u8> {
        assert_eq!(self.bit_depth, BitDepth::Eight);
        assert_eq!(self.color_type, ColorType::Indexed);
        self.data.to_vec()
    }

    pub fn as_ci4(&self) -> Vec<u8> {
        assert_eq!(self.color_type, ColorType::Indexed);

        match self.bit_depth {
            BitDepth::Four => self.data.to_vec(),
            BitDepth::Eight => self
                .data
                .chunks_exact(2)
                .map(|chunk| chunk[0] << 4 | chunk[1])
                .collect(),
            _ => panic!("unsupported bit depth: {:?}", self.bit_depth),
        }
    }

    pub fn as_i4(&self) -> Vec<u8> {
        match (self.color_type, self.bit_depth) {
            (ColorType::Grayscale, BitDepth::Four) => self.data.to_vec(),
            (ColorType::Grayscale, BitDepth::Eight) => self
                .data
                .chunks_exact(2)
                .map(|chunk| u8_to_u4(chunk[0]) << 4 | u8_to_u4(chunk[1]))
                .collect(),
            (ColorType::Rgba, BitDepth::Eight) => self
                .data
                .chunks_exact(8)
                .map(|chunk| {
                    let c1 = Color::RGBA(chunk[0], chunk[1], chunk[2], chunk[3]);
                    let i1 = c1.rgb_to_intensity();
                    let c2 = Color::RGBA(chunk[4], chunk[5], chunk[6], chunk[7]);
                    let i2 = c2.rgb_to_intensity();

                    u8_to_u4(i1) << 4 | u8_to_u4(i2)
                })
                .collect(),
            (ColorType::Rgb, BitDepth::Eight) => self
                .data
                .chunks_exact(6)
                .map(|chunk| {
                    let c1 = Color::RGB(chunk[0], chunk[1], chunk[2]);
                    let i1 = c1.rgb_to_intensity();
                    let c2 = Color::RGB(chunk[3], chunk[4], chunk[5]);
                    let i2 = c2.rgb_to_intensity();

                    u8_to_u4(i1) << 4 | u8_to_u4(i2)
                })
                .collect(),
            p => panic!("unsupported format {:?}", p),
        }
    }

    pub fn as_i8(&self) -> Vec<u8> {
        match (self.color_type, self.bit_depth) {
            (ColorType::Grayscale, BitDepth::Eight) => self.data.to_vec(),
            (ColorType::Grayscale, BitDepth::Four) => self
                .data
                .chunks_exact(2)
                .map(|chunk| chunk[0] << 4 | chunk[1])
                .collect(),
            (ColorType::Rgba, BitDepth::Eight) => self
                .data
                .chunks_exact(4)
                .map(|chunk| {
                    let c = Color::RGBA(chunk[0], chunk[1], chunk[2], chunk[3]);
                    c.rgb_to_intensity()
                })
                .collect(),
            (ColorType::Rgb, BitDepth::Eight) => self
                .data
                .chunks_exact(3)
                .map(|chunk| {
                    let c = Color::RGB(chunk[0], chunk[1], chunk[2]);
                    c.rgb_to_intensity()
                })
                .collect(),
            p => panic!("unsupported format {:?}", p),
        }
    }

    pub fn as_ia4(&self) -> Vec<u8> {
        match (self.color_type, self.bit_depth) {
            (ColorType::GrayscaleAlpha, BitDepth::Eight) => self
                .data
                .chunks_exact(4)
                .map(|chunk| {
                    let intensity = (chunk[0] >> 5) << 1;
                    let alpha = (chunk[1] > 127) as u8;
                    let high = intensity | alpha;

                    let intensity = (chunk[2] >> 5) << 1;
                    let alpha = (chunk[3] > 127) as u8;
                    let low = intensity | alpha;

                    high << 4 | (low & 0xF)
                })
                .collect(),
            p => panic!("unsupported format {:?}", p),
        }
    }

    pub fn as_ia8(&self) -> Vec<u8> {
        match (self.color_type, self.bit_depth) {
            (ColorType::GrayscaleAlpha, BitDepth::Eight) => self
                .data
                .chunks_exact(2)
                .map(|chunk| chunk[0] << 4 | (chunk[1] & 0x0F))
                .collect(),
            p => panic!("unsupported format {:?}", p),
        }
    }

    pub fn as_ia16(&self) -> Vec<u8> {
        match (self.color_type, self.bit_depth) {
            (ColorType::GrayscaleAlpha, BitDepth::Eight) => self.data.to_vec(),
            p => panic!("unsupported format {:?}", p),
        }
    }

    pub fn as_rgba16(&self) -> Vec<u8> {
        match (self.color_type, self.bit_depth) {
            (ColorType::Rgba, BitDepth::Eight) => self
                .data
                .chunks_exact(4)
                .flat_map(|chunk| {
                    let color = Color::RGBA(chunk[0], chunk[1], chunk[2], chunk[3]);
                    let (high, low) = color.rgba16();
                    vec![high, low]
                })
                .collect(),
            p => panic!("unsupported format {:?}", p),
        }
    }

    pub fn as_rgba32(&self) -> Vec<u8> {
        match (self.color_type, self.bit_depth) {
            (ColorType::Rgba, BitDepth::Eight) => self.data.to_vec(),
            p => panic!("unsupported format {:?}", p),
        }
    }
}

pub fn create_palette_from_png<R: Read>(r: R) -> Vec<u8> {
    let decoder = png::Decoder::new(r);
    let reader = decoder.read_info().unwrap();
    let info = reader.info();
    let rgb_data = info.palette.as_ref().expect("given PNG has no palette");
    let alpha_data = info.trns.as_ref();

    match alpha_data {
        Some(alpha_data) => rgb_data
            .chunks_exact(3)
            .zip(alpha_data.iter())
            .flat_map(|(rgb, &alpha)| {
                let color = Color::RGBA(rgb[0], rgb[1], rgb[2], alpha);
                let (high, low) = color.rgba16();
                vec![high, low]
            })
            .collect(),

        // If there's no alpha channel, assume everything is opaque
        None => rgb_data
            .chunks_exact(3)
            .flat_map(|rgb| {
                let color = Color::RGB(rgb[0], rgb[1], rgb[2]);
                let (high, low) = color.rgba16();
                vec![high, low]
            })
            .collect(),
    }
}
