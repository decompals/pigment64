use anyhow::Result;
use pigment64::image::native_image::parse_tlut;
use pigment64::{ImageSize, ImageType, NativeImage, PNGImage, TextureLUT, create_palette_from_png};
use strum::{EnumCount, IntoEnumIterator};

#[test]
fn ci4() -> Result<()> {
    let original_bytes: &[u8] = include_bytes!("ci4.data.bin");
    let image = NativeImage::read(original_bytes, ImageType::Ci4, 4, 4)?;

    let tlut_bytes: &[u8] = include_bytes!("ci4.tlut.bin");
    let tlut_table: Vec<u8> = parse_tlut(tlut_bytes, ImageSize::Bits4, TextureLUT::Rgba16)?;

    let mut output: Vec<u8> = Vec::new();
    image.as_png(&mut output, Some(tlut_table.as_slice()))?;

    // convert the png back to a native image
    let image = PNGImage::read(output.as_slice())?;
    let mut output_bytes: Vec<u8> = Vec::new();
    image.as_ci4(&mut output_bytes)?;

    // convert the png back to a texture lut
    let mut output_tlut: Vec<u8> = Vec::new();
    create_palette_from_png(output.as_slice(), &mut output_tlut)?;

    assert_eq!(output_bytes, original_bytes);
    assert_eq!(output_tlut, tlut_bytes);
    Ok(())
}

#[test]
fn i1() -> Result<()> {
    let original_bytes: &[u8] = include_bytes!("i1.png.bin");
    let image = NativeImage::read(original_bytes, ImageType::I1, 72, 72)?;

    let mut output: Vec<u8> = Vec::new();
    image.as_png(&mut output, None)?;

    // convert the png back to a native image
    let image = PNGImage::read(output.as_slice())?;
    let mut output_bytes: Vec<u8> = Vec::new();
    image.as_i1(&mut output_bytes)?;

    assert_eq!(output_bytes, original_bytes);
    Ok(())
}

#[test]
fn i4() -> Result<()> {
    let original_bytes: &[u8] = include_bytes!("i4.png.bin");
    let image = NativeImage::read(original_bytes, ImageType::I4, 16, 1)?;

    let mut output: Vec<u8> = Vec::new();
    image.as_png(&mut output, None)?;

    // convert the png back to a native image
    let image = PNGImage::read(output.as_slice())?;
    let mut output_bytes: Vec<u8> = Vec::new();
    image.as_i4(&mut output_bytes)?;

    assert_eq!(output_bytes, original_bytes);
    Ok(())
}

#[test]
fn i8() -> Result<()> {
    let original_bytes: &[u8] = include_bytes!("i8.png.bin");
    let image = NativeImage::read(original_bytes, ImageType::I8, 16, 16)?;

    let mut output: Vec<u8> = Vec::new();
    image.as_png(&mut output, None)?;

    // convert the png back to a native image
    let image = PNGImage::read(output.as_slice())?;
    let mut output_bytes: Vec<u8> = Vec::new();
    image.as_i8(&mut output_bytes)?;

    assert_eq!(output_bytes, original_bytes);
    Ok(())
}

#[test]
fn ia4() -> Result<()> {
    let original_bytes: &[u8] = include_bytes!("ia4.png.bin");
    let image = NativeImage::read(original_bytes, ImageType::Ia4, 16, 1)?;

    let mut output: Vec<u8> = Vec::new();
    image.as_png(&mut output, None)?;

    // convert the png back to a native image
    let image = PNGImage::read(output.as_slice())?;
    let mut output_bytes: Vec<u8> = Vec::new();
    image.as_ia4(&mut output_bytes)?;

    assert_eq!(output_bytes, original_bytes);
    Ok(())
}

#[test]
fn ia8() -> Result<()> {
    let original_bytes: &[u8] = include_bytes!("ia8.png.bin");
    let image = NativeImage::read(original_bytes, ImageType::Ia8, 16, 16)?;

    let mut output: Vec<u8> = Vec::new();
    image.as_png(&mut output, None)?;

    // convert the png back to a native image
    let image = PNGImage::read(output.as_slice())?;
    let mut output_bytes: Vec<u8> = Vec::new();
    image.as_ia8(&mut output_bytes)?;

    assert_eq!(output_bytes, original_bytes);
    Ok(())
}

#[test]
fn ia16() -> Result<()> {
    let original_bytes: &[u8] = include_bytes!("ia16.png.bin");
    let image = NativeImage::read(original_bytes, ImageType::Ia16, 256, 256)?;

    let mut output: Vec<u8> = Vec::new();
    image.as_png(&mut output, None)?;

    // convert the png back to a native image
    let image = PNGImage::read(output.as_slice())?;
    let mut output_bytes: Vec<u8> = Vec::new();
    image.as_ia16(&mut output_bytes)?;

    assert_eq!(output_bytes, original_bytes);
    Ok(())
}

#[test]
fn rgba16() -> Result<()> {
    let original_bytes: &[u8] = include_bytes!("rgba16.png.bin");
    let image = NativeImage::read(original_bytes, ImageType::Rgba16, 256, 256)?;

    let mut output: Vec<u8> = Vec::new();
    image.as_png(&mut output, None)?;

    // convert the png back to a native image
    let image = PNGImage::read(output.as_slice())?;
    let mut output_bytes: Vec<u8> = Vec::new();
    image.as_rgba16(&mut output_bytes)?;

    assert_eq!(output_bytes, original_bytes);
    Ok(())
}

#[test]
fn rgba32() -> Result<()> {
    let original_bytes: &[u8] = include_bytes!("rgba32.png.bin");
    let image = NativeImage::read(original_bytes, ImageType::Rgba32, 32, 32)?;

    let mut output: Vec<u8> = Vec::new();
    image.as_png(&mut output, None)?;

    // convert the png back to a native image
    let image = PNGImage::read(output.as_slice())?;
    let mut output_bytes: Vec<u8> = Vec::new();
    image.as_rgba32(&mut output_bytes)?;

    assert_eq!(output_bytes, original_bytes);
    Ok(())
}

#[test]
fn test_image_type_strum() {
    // Test iterating over the ImageType enum
    let mut image_iter = ImageType::iter();
    assert_eq!(Some(ImageType::I1), image_iter.next());
    assert_eq!(Some(ImageType::I4), image_iter.next());
    assert_eq!(Some(ImageType::I8), image_iter.next());
    assert_eq!(Some(ImageType::Ia4), image_iter.next());
    assert_eq!(Some(ImageType::Ia8), image_iter.next());
    assert_eq!(Some(ImageType::Ia16), image_iter.next());
    assert_eq!(Some(ImageType::Ci4), image_iter.next());
    assert_eq!(Some(ImageType::Ci8), image_iter.next());
    assert_eq!(Some(ImageType::Rgba16), image_iter.next());
    assert_eq!(Some(ImageType::Rgba32), image_iter.next());
    assert_eq!(None, image_iter.next());

    // Test the Correct number of items
    assert_eq!(10, ImageType::COUNT);
    assert_eq!(ImageType::iter().count(), ImageType::COUNT);
}

#[test]
fn test_swap_word_rows() {
    // Create a dummy 4x4 32-bit RGBA image.
    // Each pixel is 4 bytes, so each row is 16 bytes.
    let image_type = pigment64::ImageType::Rgba32;
    let width = 4;
    let height = 4;

    // Create some initial data.
    // Rows 0 and 2 should remain unchanged.
    // Rows 1 and 3 should have their words swapped.
    let initial_data: Vec<u8> = vec![
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
        0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D,
        0x1E, 0x1F, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x2B, 0x2C,
        0x2D, 0x2E, 0x2F, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A, 0x3B,
        0x3C, 0x3D, 0x3E, 0x3F,
    ];

    let mut image = pigment64::NativeImage {
        format: image_type,
        width,
        height,
        data: initial_data,
    };
    image.swap_word_rows();

    let expected_data: Vec<u8> = vec![
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
        0x0F, 0x14, 0x15, 0x16, 0x17, 0x10, 0x11, 0x12, 0x13, 0x1C, 0x1D, 0x1E, 0x1F, 0x18, 0x19,
        0x1A, 0x1B, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x2B, 0x2C,
        0x2D, 0x2E, 0x2F, 0x34, 0x35, 0x36, 0x37, 0x30, 0x31, 0x32, 0x33, 0x3C, 0x3D, 0x3E, 0x3F,
        0x38, 0x39, 0x3A, 0x3B,
    ];

    assert_eq!(image.data, expected_data);
}
