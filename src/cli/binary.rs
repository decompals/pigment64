use crate::cli::defines::BinaryFormat;
use crate::write_buf_as_raw_array;
use anyhow::Result;
use clap::{Args, ValueEnum};
use pigment64::Error;
use std::{
    fs::File,
    io::{self, BufReader, BufWriter, Write},
    mem,
    path::PathBuf,
};

// MARK: - Args

#[derive(Args, Debug)]
pub struct BinaryArgs {
    /// Path to the PNG input file
    input: String,

    /// Output file. Defaults to input file name with ".bin" appended
    #[arg(short)]
    output: Option<String>,

    /// Output format
    #[arg(value_enum, short, long)]
    format: BinaryFormat,

    /// Flip the image on the x axis
    #[arg(long)]
    flip_x: bool,

    /// Flip the image on the y axis
    #[arg(long)]
    flip_y: bool,

    /// Swap words in odd rows
    #[arg(long)]
    word_swap: bool,

    /// Output a raw C array which can be `#include`d in a file. The default output type width matches the FORMAT provided, but it can be overridden with --c_array_width
    #[arg(long)]
    c_array: bool,

    /// Overrides the natural fit of each format when outputting a C array
    #[arg(long, value_enum)]
    c_array_width: Option<CArrayWidth>,
}

// MARK: - Handlers

pub fn handle_binary(args: &BinaryArgs) -> Result<()> {
    let input_file = File::open(&args.input).expect("could not open input file");
    let mut input_reader = BufReader::new(input_file);

    // Convert the image
    let mut bin: Vec<u8> = Vec::new();

    if let BinaryFormat::Palette = args.format {
        pigment64::create_palette_from_png(&mut input_reader, &mut bin)?;
    } else {
        let mut image = pigment64::PNGImage::read(&mut input_reader)?;

        if args.flip_x || args.flip_y {
            image = image.flip(args.flip_x, args.flip_y);
        }

        let image_type = args
            .format
            .as_native()
            .ok_or(Error::PaletteConversionError)?;

        image.as_native(&mut bin, image_type)?;

        if args.word_swap {
            let mut native_image = pigment64::NativeImage {
                format: image_type,
                width: image.width(),
                height: image.height(),
                data: bin,
            };
            native_image.swap_word_rows();
            bin = native_image.data;
        }
    };

    let mut output_file: Box<dyn Write>;

    if args.c_array && args.output.is_none() {
        output_file = Box::from(io::stdout());
    } else {
        let output_path = PathBuf::from(args.output.clone().unwrap_or_else(|| {
            let mut path = args.input.clone();
            if args.c_array {
                path.push_str(".inc.c");
            } else {
                path.push_str(".bin");
            }
            path
        }));

        let file = File::create(output_path)?;
        output_file = Box::from(file);
    }

    if args.c_array {
        // Override array width if the user passed the appropriate flag
        let c_array_width = args.c_array_width.unwrap_or(args.format.get_width());

        match c_array_width {
            CArrayWidth::U8 => write_buf_as_u8(&mut output_file, &bin),
            CArrayWidth::U16 => write_buf_as_u16(&mut output_file, &bin),
            CArrayWidth::U32 => write_buf_as_u32(&mut output_file, &bin),
            CArrayWidth::U64 => write_buf_as_u64(&mut output_file, &bin),
        }
    } else {
        BufWriter::new(output_file).write_all(&bin)?;
    }

    Ok(())
}

// MARK: - Structs

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
pub enum CArrayWidth {
    U8,
    U16,
    U32,
    U64,
}

// MARK: - Helpers

fn write_buf_as_u8(output_file: &mut Box<dyn Write>, bin: &[u8]) {
    write_buf_as_raw_array!(output_file, bin, u8);
}

fn write_buf_as_u16(output_file: &mut Box<dyn Write>, bin: &[u8]) {
    write_buf_as_raw_array!(output_file, bin, u16);
}

fn write_buf_as_u32(output_file: &mut Box<dyn Write>, bin: &[u8]) {
    write_buf_as_raw_array!(output_file, bin, u32);
}

fn write_buf_as_u64(output_file: &mut Box<dyn Write>, bin: &[u8]) {
    write_buf_as_raw_array!(output_file, bin, u64);
}
