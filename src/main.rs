use clap::{Parser, ValueEnum};
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

/// PNG to N64 image converter
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input PNG file
    input: String,

    /// Output file. Defaults to input file name with ".bin" appended
    #[arg(short)]
    output: Option<String>,

    /// Output format
    #[arg(value_enum)]
    format: Format,

    /// Flip the image on the x axis
    #[arg(long)]
    flip_x: bool,

    /// Flip the image on the y axis
    #[arg(long)]
    flip_y: bool,

    /// Output a raw C array which can be `#include`d in a file. The default output type width matches the FORMAT provided, but it can be overridden with --type-width
    #[arg(short, long)]
    c_array: bool,

    /// Overrides the natural fit of each format when outputting a C array
    #[arg(short, long, value_enum)]
    type_width: Option<TypeWideArray>,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
enum Format {
    Ci4,
    Ci8,
    I4,
    I8,
    Ia4,
    Ia8,
    Ia16,
    Rgba16,
    Rgba32,
    Palette,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
enum TypeWideArray {
    U8,
    U16,
    U32,
    U64,
}

#[macro_export]
macro_rules! write_buf_as_raw_array {
    ($dst:expr, $bin:expr, $width:expr, $callback:expr) => {
        for row in $bin.chunks(16) {
            let mut line_list = Vec::new();
            for bytes in row.chunks($width) {
                let value = $callback(bytes.try_into().unwrap());
                line_list.push(format!("0x{value:00$X}", 2*$width));
            }
            let line = line_list.join(", ");
            write!($dst, "    {line},\n").expect("could not write to output file");
        }
    }
}

fn write_buf_as_u8(output_file: &mut File, bin: &Vec<u8>) {
    write_buf_as_raw_array!(output_file, bin, 1, u8::from_be_bytes);
}

fn write_buf_as_u16(output_file: &mut File, bin: &Vec<u8>) {
    write_buf_as_raw_array!(output_file, bin, 2, u16::from_be_bytes);
}

fn write_buf_as_u32(output_file: &mut File, bin: &Vec<u8>) {
    write_buf_as_raw_array!(output_file, bin, 4, u32::from_be_bytes);
}

fn write_buf_as_u64(output_file: &mut File, bin: &Vec<u8>) {
    write_buf_as_raw_array!(output_file, bin, 8, u64::from_be_bytes);
}

fn main() {
    let args = Args::parse();

    let input_file = File::open(&args.input).expect("could not open input file");
    let mut input_reader = BufReader::new(input_file);

    // Convert the image
    let bin = if let Format::Palette = args.format {
        pigment::get_palette_rgba16(&mut input_reader)
    } else {
        let mut image = pigment::Image::read_png(&mut input_reader);

        if args.flip_x || args.flip_y {
            image = image.flip(args.flip_x, args.flip_y);
        }

        match args.format {
            Format::Ci4 => image.as_ci4(),
            Format::Ci8 => image.as_ci8(),
            Format::I4 => image.as_i4(),
            Format::I8 => image.as_i8(),
            Format::Ia4 => image.as_ia4(),
            Format::Ia8 => image.as_ia8(),
            Format::Ia16 => image.as_ia16(),
            Format::Rgba16 => image.as_rgba16(),
            Format::Rgba32 => image.as_rgba32(),
            Format::Palette => unreachable!(),
        }
    };

    let output_path = PathBuf::from(args.output.unwrap_or_else(|| {
        let mut path = args.input.clone();
        path.push_str(".bin");
        path
    }));

    let mut output_file = File::create(output_path).expect("could not create output file");

    if args.c_array {
        let mut type_width: TypeWideArray;

        // Compute the default value first
        match args.format {
            Format::Ci4 => type_width = TypeWideArray::U8,
            Format::Ci8 => type_width = TypeWideArray::U8,
            Format::I4 => type_width = TypeWideArray::U8,
            Format::I8 => type_width = TypeWideArray::U8,
            Format::Ia4 => type_width = TypeWideArray::U8,
            Format::Ia8 => type_width = TypeWideArray::U8,
            Format::Ia16 => type_width = TypeWideArray::U16,
            Format::Rgba16 => type_width = TypeWideArray::U16,
            Format::Rgba32 => type_width = TypeWideArray::U32,
            Format::Palette => type_width = TypeWideArray::U16,
        }

        // Override if the user passed the appropriate flag
        type_width = args.type_width.unwrap_or(type_width);

        match type_width {
            TypeWideArray::U8 => write_buf_as_u8(&mut output_file, &bin),
            TypeWideArray::U16 => write_buf_as_u16(&mut output_file, &bin),
            TypeWideArray::U32 => write_buf_as_u32(&mut output_file, &bin),
            TypeWideArray::U64 => write_buf_as_u64(&mut output_file, &bin),
        }
    } else {
        BufWriter::new(output_file)
            .write_all(&bin)
            .expect("could not write to output file");
    }
}
