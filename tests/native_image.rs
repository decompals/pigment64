use anyhow::Result;
use pigment64::{ImageType, NativeImage};

const DEBUG: bool = false;

fn _write_generated_image(data: &[u8], name: &str) -> Result<()> {
    if DEBUG == false {
        return Ok(());
    }

    use std::fs::File;
    use std::io::Write;
    let mut file = File::create(name)?;
    file.write_all(data)?;

    Ok(())
}

#[test]
fn i8() -> Result<()> {
    let bytes: &[u8] = include_bytes!("i8.png.bin");
    let image = NativeImage::read(bytes, ImageType::I8, 16, 16)?;

    let expected_bytes = include_bytes!("i8.png");
    let mut output: Vec<u8> = Vec::new();
    image.as_png(&mut output, None)?;

    // write png to file for debugging
    _write_generated_image(&output, "i8.result.png")?;

    assert_eq!(output, expected_bytes);
    Ok(())
}

#[test]
fn ia8() -> Result<()> {
    let bytes: &[u8] = include_bytes!("ia8.png.bin");
    let image = NativeImage::read(bytes, ImageType::Ia16, 16, 16)?;

    let expected_bytes = include_bytes!("ia8.png");
    let mut output: Vec<u8> = Vec::new();
    image.as_png(&mut output, None)?;

    // write png to file for debugging
    _write_generated_image(&output, "ia8.result.png")?;

    assert_eq!(output, expected_bytes);
    Ok(())
}

#[test]
fn ia16() -> Result<()> {
    let bytes: &[u8] = include_bytes!("ia16.png.bin");
    let image = NativeImage::read(bytes, ImageType::Ia16, 256, 256)?;

    let expected_bytes = include_bytes!("ia16.png");
    let mut output: Vec<u8> = Vec::new();
    image.as_png(&mut output, None)?;

    // write png to file for debugging
    _write_generated_image(&output, "ia16.result.png")?;

    assert_eq!(output, expected_bytes);
    Ok(())
}

#[test]
fn rgba16() -> Result<()> {
    let bytes: &[u8] = include_bytes!("rgba16.png.bin");
    let image = NativeImage::read(bytes, ImageType::Rgba16, 256, 256)?;

    let expected_bytes = include_bytes!("rgba16.png");
    let mut output: Vec<u8> = Vec::new();
    image.as_png(&mut output, None)?;

    // write png to file for debugging
    _write_generated_image(&output, "rgba16.result.png")?;

    assert_eq!(output, expected_bytes);
    Ok(())
}

#[test]
fn rgba32() -> Result<()> {
    let bytes: &[u8] = include_bytes!("rgba32.png.bin");
    let image = NativeImage::read(bytes, ImageType::Rgba32, 32, 32)?;

    let expected_bytes = include_bytes!("rgba32.png");
    let mut output: Vec<u8> = Vec::new();
    image.as_png(&mut output, None)?;

    // write png to file for debugging
    _write_generated_image(&output, "rgba32.result.png")?;

    assert_eq!(output, expected_bytes);
    Ok(())
}
