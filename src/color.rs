/// RGBA color type where values are 8-bit integers (0-255).
///
/// This is not to be used as a generic color type, only for specific pigment interfaces.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    #[rustfmt::skip]
    pub const TRANSPARENT: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    #[rustfmt::skip]
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0, a: 255 };
    #[rustfmt::skip]
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255, a: 255 };

    /// Creates a new color with the given RGBA values.
    #[inline]
    #[allow(non_snake_case)]
    pub const fn RGBA(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }

    /// Creates a new color with the given RGB values and an alpha value of 255.
    #[inline]
    #[allow(non_snake_case)]
    pub const fn RGB(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 0xFF }
    }

    /// Creates a new color from a 32-bit RGBA pixel.
    #[inline]
    pub fn from_u32(pixel: u32) -> Color {
        let r = ((pixel >> 24) & 0xFF) as u8;
        let g = ((pixel >> 16) & 0xFF) as u8;
        let b = ((pixel >> 8) & 0xFF) as u8;
        let a = (pixel & 0xFF) as u8;

        Color { r, g, b, a }
    }

    /// Converts a 16-bit RGBA pixel to a 32-bit RGBA color.
    #[inline]
    pub fn from_u16(pixel: u16) -> Color {
        let r = ((pixel >> 11) & 0x1F) as u8;
        let g = ((pixel >> 6) & 0x1F) as u8;
        let b = ((pixel >> 1) & 0x1F) as u8;
        let a = (pixel & 0x01) as u8;

        let r = (r << 3) | (r >> 2);
        let g = (g << 3) | (g >> 2);
        let b = (b << 3) | (b >> 2);
        let a = 255 * a;

        Color { r, g, b, a }
    }

    /// Converts a 32-bit RGBA color to a 16-bit RGBA pixel.
    #[inline]
    pub fn to_u16(&self) -> u16 {
        let r = (self.r / 8) as u16;
        let g = (self.g / 8) as u16;
        let b = (self.b / 8) as u16;
        let a = (self.a / 255) as u16;

        (r << 11) | (g << 6) | (b << 1) | a
    }

    /// Converts a 32-bit RGBA color to a 16-bit RGBA pixel and
    /// returns the two 8-bit components.
    #[inline]
    pub fn rgba16(self) -> [u8; 2] {
        let pixel = self.to_u16();
        [(pixel >> 8) as u8, (pixel & 0xFF) as u8]
    }

    /// Converts the rgb components to a single intensity value.
    /// This is used for grayscale images.
    #[inline]
    pub fn rgb_to_intensity(&self) -> u8 {
        (self.r as f32 * 0.2126 + self.g as f32 * 0.7152 + 0.0722 * self.b as f32).round() as u8
    }
}
