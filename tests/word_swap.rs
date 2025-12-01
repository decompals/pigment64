use assert_cmd::Command;
use pigment64::{ImageType, image::png_image::PNGImage};
use std::fs;

fn get_asset_path(asset: &str) -> String {
    format!("{}/tests/{}", env!("CARGO_MANIFEST_DIR"), asset)
}

fn test_swap_logic(image_type: ImageType, asset_name: &str) {
    let base_name = asset_name.strip_suffix(".png").unwrap_or(asset_name);
    let input_png_path = get_asset_path(asset_name);
    let generated_swapped_bin_path = get_asset_path(&format!("{}.word_swap.bin", base_name));
    let original_unswapped_bin_path = get_asset_path(&format!("{}.png.bin", base_name));
    let generated_png_path = get_asset_path(&format!("{}.word_swap.png", base_name));

    // Get image dimensions
    let png_file = fs::File::open(&input_png_path).expect("Failed to open input PNG");
    let png = PNGImage::read(png_file).expect("Failed to read PNG");
    let (width, height) = (png.width(), png.height());

    // Convert PNG to binary
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_pigment64_cli"));
    cmd.args([
        "to-bin",
        &input_png_path,
        "-o",
        &generated_swapped_bin_path,
        "-f",
        match image_type {
            ImageType::I4 => "i4",
            ImageType::Rgba32 => "rgba32",
            _ => panic!("Unsupported type for this test helper"),
        },
        "--word-swap",
    ])
    .assert()
    .success();

    // Create EXPECTED data by manually swapping the known good unswapped bin
    let mut expected_swapped_data =
        fs::read(&original_unswapped_bin_path).expect("Failed to read reference bin");

    let bpp = image_type.get_size().get_bpp();
    // Ceiling division handles widths that aren't byte-aligned
    let bytes_per_row = (width * bpp).div_ceil(8);
    let bytes_per_row = bytes_per_row as usize;

    for y in (1..height).step_by(2) {
        let row_start = (y as usize) * bytes_per_row;
        let row_end = row_start + bytes_per_row;

        if row_end > expected_swapped_data.len() {
            break;
        }

        let row_data = &mut expected_swapped_data[row_start..row_end];

        for chunk in row_data.chunks_mut(8) {
            if chunk.len() == 8 {
                let (word1, word2) = chunk.split_at_mut(4);
                word1.swap_with_slice(word2);
            }
        }
    }

    // Compare CLI output with Manual Expectation
    let generated_swapped_data =
        fs::read(&generated_swapped_bin_path).expect("Failed to read generated bin");
    assert_eq!(
        generated_swapped_data, expected_swapped_data,
        "Binary data mismatch for {}",
        asset_name
    );

    // Test converting back (Round trip)
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_pigment64_cli"));
    cmd.args([
        "to-png",
        &generated_swapped_bin_path,
        "-o",
        &generated_png_path,
        "-f",
        match image_type {
            ImageType::I4 => "i4",
            ImageType::Rgba32 => "rgba32",
            _ => "unknown",
        },
        "--width",
        &width.to_string(),
        "--height",
        &height.to_string(),
        "--word-swap",
    ])
    .assert()
    .success();

    let generated_png_file = fs::File::open(&generated_png_path).unwrap();
    let generated_png = PNGImage::read(generated_png_file).unwrap();
    let mut generated_unswapped_bin_data = Vec::new();
    generated_png
        .as_native(&mut generated_unswapped_bin_data, image_type)
        .unwrap();

    let original_unswapped_bin_data = fs::read(original_unswapped_bin_path).unwrap();
    assert_eq!(
        generated_unswapped_bin_data, original_unswapped_bin_data,
        "Round trip mismatch for {}",
        asset_name
    );

    // Cleanup
    let _ = fs::remove_file(&generated_swapped_bin_path);
    let _ = fs::remove_file(&generated_png_path);
}

#[test]
fn test_word_swap_i4() {
    test_swap_logic(ImageType::I4, "i4.png");
}

#[test]
fn test_word_swap_rgba32() {
    test_swap_logic(ImageType::Rgba32, "rgba32.png");
}
