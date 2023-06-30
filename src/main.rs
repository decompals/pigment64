use clap::{Parser, ValueEnum};
use format::OutputMethod;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

mod format;

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

    /// Output method
    #[arg(value_enum, default_value_t = OutputMethod::Bin)]
    output_method: OutputMethod,

    /// C array name
    #[arg(short)]
    name: Option<String>,

    /// Element width
    #[arg(short)]
    elem_width: Option<usize>,

    /// Type width
    #[arg(short)]
    type_width: Option<usize>,

    /// Elements per line
    #[arg(short, default_value_t = 8)]
    line_width: usize,

    /// Endianness
    #[arg(long = "end", value_enum, default_value_t = Endianness::Big)]
    endianness: Endianness,

    /// Flip the image on the x axis
    #[arg(long)]
    flip_x: bool,

    /// Flip the image on the y axis
    #[arg(long)]
    flip_y: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
enum Endianness {
    Little,
    Big,
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

impl Format {
    fn get_width(&self) -> usize {
        match self {
            Format::Ci4 => 1,
            Format::Ci8 => 1,
            Format::I4 => 1,
            Format::I8 => 1,
            Format::Ia4 => 1,
            Format::Ia8 => 1,
            Format::Ia16 => 2,
            Format::Rgba16 => 2,
            Format::Rgba32 => 4,
            Format::Palette => 2,
        }
    }
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
        path.push_str(&format!(".{}", args.output_method.get_extension()));
        path
    }));

    let elem_width = args.elem_width.unwrap_or(args.format.get_width());

    BufWriter::new(File::create(&output_path).expect("could not open output file"))
        .write_all(
            &args.output_method.encode(
                bin,
                &args.name.unwrap_or(
                    output_path
                        .file_stem()
                        .and_then(|n| n.to_str())
                        .unwrap_or("data")
                        .to_string(),
                ),
                elem_width,
                args.type_width.unwrap_or(elem_width),
                args.line_width,
                args.endianness == Endianness::Little,
            ),
        )
        .expect("could not write to output file");
}
