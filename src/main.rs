use anyhow::Result;
use clap::{Parser, ValueEnum};
use pigment64::ImageType;
use std::fs::File;
use std::io::{self, prelude::*};
use std::io::{BufReader, BufWriter};
use std::mem;
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
    format: OutputFormat,

    /// Flip the image on the x axis
    #[arg(long)]
    flip_x: bool,

    /// Flip the image on the y axis
    #[arg(long)]
    flip_y: bool,

    /// Output a raw C array which can be `#include`d in a file. The default output type width matches the FORMAT provided, but it can be overridden with --c_array_width
    #[arg(short, long)]
    c_array: bool,

    /// Overrides the natural fit of each format when outputting a C array
    #[arg(short, long, value_enum)]
    c_array_width: Option<CArrayWidth>,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
enum OutputFormat {
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

impl OutputFormat {
    fn get_width(&self) -> CArrayWidth {
        match self {
            OutputFormat::Ci4 => CArrayWidth::U8,
            OutputFormat::Ci8 => CArrayWidth::U8,
            OutputFormat::I4 => CArrayWidth::U8,
            OutputFormat::I8 => CArrayWidth::U8,
            OutputFormat::Ia4 => CArrayWidth::U8,
            OutputFormat::Ia8 => CArrayWidth::U8,
            OutputFormat::Ia16 => CArrayWidth::U16,
            OutputFormat::Rgba16 => CArrayWidth::U16,
            OutputFormat::Rgba32 => CArrayWidth::U32,
            OutputFormat::Palette => CArrayWidth::U16,
        }
    }

    fn as_native(&self) -> ImageType {
        match self {
            OutputFormat::Ci4 => ImageType::Ci4,
            OutputFormat::Ci8 => ImageType::Ci8,
            OutputFormat::I4 => ImageType::I4,
            OutputFormat::I8 => ImageType::I8,
            OutputFormat::Ia4 => ImageType::Ia4,
            OutputFormat::Ia8 => ImageType::Ia8,
            OutputFormat::Ia16 => ImageType::Ia16,
            OutputFormat::Rgba16 => ImageType::Rgba16,
            OutputFormat::Rgba32 => ImageType::Rgba32,
            _ => panic!("cannot convert palette to native format"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
enum CArrayWidth {
    U8,
    U16,
    U32,
    U64,
}

#[macro_export]
macro_rules! write_buf_as_raw_array {
    ($dst:expr, $bin:expr, $type_width:ident) => {
        let width = mem::size_of::<$type_width>();

        for row in $bin.chunks(16) {
            let mut line_list = Vec::new();
            for bytes in row.chunks(width) {
                let value = $type_width::from_be_bytes(bytes.try_into().unwrap());

                line_list.push(format!("0x{value:00$X}", 2 * width));
            }
            let line = line_list.join(", ");
            write!($dst, "    {line},\n").expect("could not write to output file");
        }
    };
}

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

fn main() -> Result<()> {
    let args = Args::parse();

    let input_file = File::open(&args.input).expect("could not open input file");
    let mut input_reader = BufReader::new(input_file);

    // Convert the image
    let mut bin: Vec<u8> = Vec::new();
    if let OutputFormat::Palette = args.format {
        pigment64::create_palette_from_png(&mut input_reader, &mut bin)?;
    } else {
        let mut image = pigment64::PNGImage::read(&mut input_reader)?;

        if args.flip_x || args.flip_y {
            image = image.flip(args.flip_x, args.flip_y);
        }

        image.as_native(&mut bin, args.format.as_native())?;
    };

    let mut output_file: Box<dyn Write>;

    if args.c_array && args.output.is_none() {
        output_file = Box::from(io::stdout());
    } else {
        let output_path = PathBuf::from(args.output.unwrap_or_else(|| {
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
        let mut c_array_width = args.format.get_width();

        // Override if the user passed the appropriate flag
        c_array_width = args.c_array_width.unwrap_or(c_array_width);

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
