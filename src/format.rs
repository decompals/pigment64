use clap::ValueEnum;

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
pub enum OutputMethod {
    Bin,
    CData,
    CFile,
}

impl OutputMethod {
    pub fn get_extension(&self) -> &str {
        match self {
            OutputMethod::Bin => "bin",
            OutputMethod::CData | OutputMethod::CFile => "c",
        }
    }

    fn num_from_slice(data: &[u8], width: usize, little_endian: bool) -> u128 {
        let mut num = 0u128;

        if width > std::mem::size_of_val(&num) {
            return 0;
        }

        if data.len() > width {
            return 0;
        }
        for byte in if little_endian {
            data.iter().rev().collect::<Vec<_>>()
        } else {
            data.iter().collect::<Vec<_>>()
        } {
            num = (num << 8) | *byte as u128;
        }

        if little_endian {
            num <<= (width - data.len()) * 8;
        }

        num
    }

    pub fn encode<T: AsRef<[u8]>>(
        &self,
        data: T,
        name: &str,
        elem_width: usize,
        type_width: usize,
        line_width: usize,
        little_endian: bool,
    ) -> Vec<u8> {
        if let OutputMethod::Bin = self {
            return data.as_ref().to_vec();
        }

        let data_len = data.as_ref().len();

        let contents = data
            .as_ref()
            .chunks(elem_width)
            .map(|e| Self::num_from_slice(e, type_width, little_endian))
            .collect::<Vec<_>>()
            .chunks(line_width)
            .map(|chunk| {
                format!(
                    "\t{}",
                    chunk
                        .iter()
                        .map(|e| format!("0x{:01$X},", e, type_width * 2))
                        .collect::<Vec<_>>()
                        .join(" ")
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        match self {
            OutputMethod::Bin => unreachable!(),
            OutputMethod::CData => contents,
            OutputMethod::CFile => format!(
                "\
#include \"common.h\"

#define {0}_LEN {1}

u{4} {2}[{0}_LEN] = {{
{3}
}};
\
                ",
                name.to_uppercase(),
                data_len / elem_width,
                name,
                contents,
                type_width * 8
            ),
        }
        .into_bytes()
    }
}
