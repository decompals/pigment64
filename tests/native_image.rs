use anyhow::Result;
use pigment64::{ImageType, NativeImage, PNGImage};

const DEBUG: bool = false;

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
