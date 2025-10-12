use crate::cli::defines::BinaryFormat;
use anyhow::Result;
use clap::Args;
use pigment64::{Error, TextureLUT, image::native_image::parse_tlut};
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

    /// Un-swap words in odd rows
    #[arg(long)]
    word_swap: bool,
}

// MARK: - Handlers

pub fn handle_png(args: &PngArgs) -> Result<()> {
    if args.format == BinaryFormat::Palette {
        return Err(Error::PaletteConversionError.into());
    }

    // Open the input file
    let input_file = File::open(&args.input)?;
    let mut input_reader = BufReader::new(input_file);

    // Convert the image
    let image_type = args
        .format
        .as_native()
        .ok_or(Error::PaletteConversionError)?;

    let mut image =
        pigment64::NativeImage::read(&mut input_reader, image_type, args.width, args.height)?;

    if args.word_swap {
        let bpp = image_type.get_size().get_bpp();
        let bytes_per_row = (image.width * bpp) / 8;

        for y in (1..image.height).step_by(2) {
            let row_start = (y * bytes_per_row) as usize;
            let row_end = row_start + bytes_per_row as usize;

            if row_end > image.data.len() {
                continue;
            }

            let row_data = &mut image.data[row_start..row_end];

            for word_pair in row_data.chunks_mut(8) {
                if word_pair.len() == 8 {
                    let (word1, word2) = word_pair.split_at_mut(4);
                    word1.swap_with_slice(word2);
                }
            }
        }
    }

    let mut output: Vec<u8> = Vec::new();

    // if format is ci4/ci8, read the palette
    if let BinaryFormat::Ci4 | BinaryFormat::Ci8 = args.format {
        let palette_path = args
            .palette
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("--palette is required for ci4/ci8 formats"))?;
        let palette_file = File::open(palette_path)?;
        let mut palette_reader = BufReader::new(palette_file);
        let mut palette_bytes = Vec::new();
        palette_reader.read_to_end(&mut palette_bytes)?;

        let image_size = args
            .format
            .get_size()
            .ok_or(Error::PaletteConversionError)?;

        let palette = parse_tlut(&palette_bytes, image_size, TextureLUT::Rgba16)?;
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
    let output_path = PathBuf::from(args.output.clone().unwrap_or_else(|| {
        let mut path = args.input.clone();
        path.push_str(".png");
        path
    }));

    let file = File::create(output_path)?;
    let mut output_writer = BufWriter::new(file);
    output_writer.write_all(&output)?;

    Ok(())
}
