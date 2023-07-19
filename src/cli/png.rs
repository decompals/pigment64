use crate::cli::defines::BinaryFormat;
use anyhow::Result;
use clap::Args;
use pigment64::image::native_image::parse_tlut;
use pigment64::TextureLUT;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;

// MARK: - Args

#[derive(Args, Debug)]
pub struct PngArgs {
    /// Path to the binary input file
    input: String,

    /// Width of the binary image
    #[arg(long)]
    width: u32,

    /// Height of the binary image
    #[arg(long)]
    height: u32,

    /// Input format
    #[arg(value_enum, short, long)]
    format: BinaryFormat,

    /// Output file. Defaults to input file name with ".png" appended
    #[arg(short, long)]
    output: Option<String>,

    /// Path to the palette binary file (only required for CI formats)
    #[arg(short, long)]
    palette: Option<String>,

    /// Flip the image on the x axis
    #[arg(long)]
    flip_x: bool,

    /// Flip the image on the y axis
    #[arg(long)]
    flip_y: bool,
}

// MARK: - Handlers

pub fn handle_png(args: &PngArgs) -> Result<()> {
    assert_ne!(args.format, BinaryFormat::Palette, "palette is not a supported standalone output format. Use format ci4/ci8 with --palette instead.");

    // Open the input file
    let input_file = File::open(&args.input).expect("could not open input file");
    let mut input_reader = BufReader::new(input_file);

    // Convert the image
    let image = pigment64::NativeImage::read(
        &mut input_reader,
        args.format.as_native(),
        args.width,
        args.height,
    )?;

    let mut output: Vec<u8> = Vec::new();

    // if format is ci4/ci8, read the palette
    if let BinaryFormat::Ci4 | BinaryFormat::Ci8 = args.format {
        // Read the palette
        let palette_file = File::open(
            args.palette
                .as_ref()
                .expect("palette is required for ci4/ci8 formats"),
        )
        .expect("could not open palette file");
        let mut palette_reader = BufReader::new(palette_file);
        let mut palette_bytes = Vec::new();
        palette_reader.read_to_end(&mut palette_bytes)?;

        let palette = parse_tlut(&palette_bytes, args.format.get_size(), TextureLUT::Rgba16)?;
        image.as_png(&mut output, Some(&palette))?;
    } else {
        image.as_png(&mut output, None)?;
    }

    // Handle flips, we do this on the already produced PNG because it's easier
    if args.flip_x || args.flip_y {
        let mut image = pigment64::PNGImage::read(&mut output.as_slice())?;
        image = image.flip(args.flip_x, args.flip_y);
        output.clear();
        image.as_png(&mut output)?;
    }

    // Write the file
    let output_file: Box<dyn Write>;
    let output_path = PathBuf::from(args.output.clone().unwrap_or_else(|| {
        let mut path = args.input.clone();
        path.push_str(".png");
        path
    }));

    let file = File::create(output_path)?;
    output_file = Box::from(file);
    BufWriter::new(output_file).write_all(&output)?;

    Ok(())
}
