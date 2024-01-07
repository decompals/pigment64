use anyhow::Result;
use pigment64::image::native_image::parse_tlut;
use pigment64::{create_palette_from_png, ImageSize, ImageType, NativeImage, PNGImage, TextureLUT};

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
    let image = NativeImage::read(original_bytes, ImageType::I1, 32, 63)?;

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
