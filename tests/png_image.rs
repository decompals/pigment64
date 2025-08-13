use anyhow::Result;
use pigment64::{PNGImage, create_palette_from_png};
use std::io::Cursor;

// TODO: convert input into all permutations of color type and bit depth
// to test all codepaths

// #[test]
// fn ci8() -> Result<()> {
//     let input_bytes = include_bytes!("ci8.png");
//     let image = Image::read_png(&mut Cursor::new(input_bytes));

//     let expected_bytes = include_bytes!("ci8.png.bin");
//     assert_eq!(image.as_ci8(), expected_bytes);
//     Ok(())
// }

#[test]
fn ci4() -> Result<()> {
    let input_bytes: &[u8] = include_bytes!("ci4.png");
    let image = PNGImage::read(input_bytes)?;

    let expected_bytes = include_bytes!("ci4.data.bin");
    let mut output: Vec<u8> = Vec::new();
    image.as_ci4(&mut output)?;

    assert_eq!(output, expected_bytes);
    Ok(())
}

#[test]
fn i1() -> Result<()> {
    let input_bytes: &[u8] = include_bytes!("i1.png");
    let image = PNGImage::read(input_bytes)?;

    let expected_bytes = include_bytes!("i1.png.bin");
    let mut output: Vec<u8> = Vec::new();
    image.as_i1(&mut output)?;

    assert_eq!(output, expected_bytes);
    Ok(())
}

#[test]
fn i4() -> Result<()> {
    let input_bytes: &[u8] = include_bytes!("i4.png");
    let image = PNGImage::read(input_bytes)?;

    let expected_bytes = include_bytes!("i4.png.bin");
    let mut output: Vec<u8> = Vec::new();
    image.as_i4(&mut output)?;

    assert_eq!(output, expected_bytes);
    Ok(())
}

#[test]
fn i8() -> Result<()> {
    let input_bytes: &[u8] = include_bytes!("i8.png");
    let image = PNGImage::read(input_bytes)?;

    let expected_bytes = include_bytes!("i8.png.bin");
    let mut output: Vec<u8> = Vec::new();
    image.as_i8(&mut output)?;

    assert_eq!(output, expected_bytes);
    Ok(())
}

#[test]
fn ia4() -> Result<()> {
    let input_bytes: &[u8] = include_bytes!("ia4.png");
    let image = PNGImage::read(input_bytes)?;

    let expected_bytes = include_bytes!("ia4.png.bin");
    let mut output: Vec<u8> = Vec::new();
    image.as_ia4(&mut output)?;

    assert_eq!(output, expected_bytes);
    Ok(())
}

#[test]
fn ia8() -> Result<()> {
    let input_bytes: &[u8] = include_bytes!("ia8.png");
    let image = PNGImage::read(&mut Cursor::new(input_bytes))?;

    let expected_bytes = include_bytes!("ia8.png.bin");
    let mut output: Vec<u8> = Vec::new();
    image.as_ia8(&mut output)?;

    assert_eq!(output, expected_bytes);
    Ok(())
}

#[test]
fn ia16() -> Result<()> {
    let input_bytes: &[u8] = include_bytes!("ia16.png");
    let image = PNGImage::read(&mut Cursor::new(input_bytes))?;

    let expected_bytes = include_bytes!("ia16.png.bin");
    let mut output: Vec<u8> = Vec::new();
    image.as_ia16(&mut output)?;

    assert_eq!(output, expected_bytes);
    Ok(())
}

#[test]
fn rgba16() -> Result<()> {
    let input_bytes: &[u8] = include_bytes!("rgba16.png");
    let image = PNGImage::read(&mut Cursor::new(input_bytes))?;

    let expected_bytes = include_bytes!("rgba16.png.bin");
    let mut output: Vec<u8> = Vec::new();
    image.as_rgba16(&mut output)?;

    assert_eq!(output, expected_bytes);
    Ok(())
}

#[test]
fn rgba32() -> Result<()> {
    let input_bytes: &[u8] = include_bytes!("rgba32.png");
    let image = PNGImage::read(&mut Cursor::new(input_bytes))?;

    let expected_bytes = include_bytes!("rgba32.png.bin");
    let mut output: Vec<u8> = Vec::new();
    image.as_rgba32(&mut output)?;

    assert_eq!(output, expected_bytes);
    Ok(())
}

#[test]
fn palette() -> Result<()> {
    let input_bytes: &[u8] = include_bytes!("ci4.png");
    let mut output_tlut: Vec<u8> = Vec::new();
    create_palette_from_png(input_bytes, &mut output_tlut)?;

    let expected_bytes = include_bytes!("ci4.tlut.bin");
    assert_eq!(output_tlut, expected_bytes);
    Ok(())
}
