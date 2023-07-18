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
