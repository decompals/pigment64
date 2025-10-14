use assert_cmd::Command;
use pigment64::{ImageType, image::png_image::PNGImage};
use std::fs;

fn get_asset_path(asset: &str) -> String {
    format!("{}/tests/{}", env!("CARGO_MANIFEST_DIR"), asset)
}

#[test]
fn test_word_swap_i4() {
    let input_png_path = get_asset_path("i4.png");
    let generated_swapped_bin_path = get_asset_path("i4.word_swap.bin");
    let original_unswapped_bin_path = get_asset_path("i4.png.bin");

    // Get image dimensions using the correct read method and public accessors
    let png_file = fs::File::open(&input_png_path).unwrap();
    let png = PNGImage::read(png_file).unwrap();
    let (width, height) = (png.width(), png.height());

    // Convert PNG to binary with word swapping
    let mut cmd = Command::cargo_bin("pigment64").unwrap();
    cmd.args([
        "to-bin",
        &input_png_path,
        "-o",
        &generated_swapped_bin_path,
        "-f",
        "i4",
        "--word-swap",
    ])
    .assert()
    .success();

    // Create the expected swapped data by swapping the original bin data manually
    let mut expected_swapped_data = fs::read(&original_unswapped_bin_path).unwrap();
    let bpp = ImageType::I4.get_size().get_bpp();
    let bytes_per_row = ((width * bpp) / 8) as usize;
    for y in (1..height).step_by(2) {
        let row_start = (y as usize) * bytes_per_row;
        let row_end = row_start + bytes_per_row;
        if row_end > expected_swapped_data.len() {
            continue;
        }
        let row_data = &mut expected_swapped_data[row_start..row_end];
        for chunk in row_data.chunks_mut(8) {
            if chunk.len() == 8 {
                let (word1, word2) = chunk.split_at_mut(4);
                word1.swap_with_slice(word2);
            }
        }
    }

    // Compare the command's output with our manually swapped data
    let generated_swapped_data = fs::read(&generated_swapped_bin_path).unwrap();
    assert_eq!(generated_swapped_data, expected_swapped_data);

    // Test converting back
    let generated_png_path = get_asset_path("i4.word_swap.png");
    let mut cmd = Command::cargo_bin("pigment64").unwrap();
    cmd.args([
        "to-png",
        &generated_swapped_bin_path,
        "-o",
        &generated_png_path,
        "-f",
        "i4",
        "--width",
        &width.to_string(),
        "--height",
        &height.to_string(),
        "--word-swap",
    ])
    .assert()
    .success();

    // Convert the resulting PNG back to native format and compare the raw data
    let generated_png_file = fs::File::open(&generated_png_path).unwrap();
    let generated_png = PNGImage::read(generated_png_file).unwrap();
    let mut generated_unswapped_bin_data = Vec::new();
    generated_png
        .as_native(&mut generated_unswapped_bin_data, ImageType::I4)
        .unwrap();

    let original_unswapped_bin_data = fs::read(original_unswapped_bin_path).unwrap();
    assert_eq!(generated_unswapped_bin_data, original_unswapped_bin_data);

    // Clean up generated files
    fs::remove_file(&generated_swapped_bin_path).unwrap();
    fs::remove_file(&generated_png_path).unwrap();
}

#[test]
fn test_word_swap_rgba32() {
    let input_png_path = get_asset_path("rgba32.png");
    let generated_swapped_bin_path = get_asset_path("rgba32.word_swap.bin");
    let original_unswapped_bin_path = get_asset_path("rgba32.png.bin");

    // Get image dimensions using the correct read method and public accessors
    let png_file = fs::File::open(&input_png_path).unwrap();
    let png = PNGImage::read(png_file).unwrap();
    let (width, height) = (png.width(), png.height());

    // Convert PNG to binary with word swapping
    let mut cmd = Command::cargo_bin("pigment64").unwrap();
    cmd.args([
        "to-bin",
        &input_png_path,
        "-o",
        &generated_swapped_bin_path,
        "-f",
        "rgba32",
        "--word-swap",
    ])
    .assert()
    .success();

    // Create the expected swapped data by swapping the original bin data manually
    let mut expected_swapped_data = fs::read(&original_unswapped_bin_path).unwrap();
    let bpp = ImageType::Rgba32.get_size().get_bpp();
    let bytes_per_row = ((width * bpp) / 8) as usize;
    for y in (1..height).step_by(2) {
        let row_start = (y as usize) * bytes_per_row;
        let row_end = row_start + bytes_per_row;
        if row_end > expected_swapped_data.len() {
            continue;
        }
        let row_data = &mut expected_swapped_data[row_start..row_end];
        for chunk in row_data.chunks_mut(8) {
            if chunk.len() == 8 {
                let (word1, word2) = chunk.split_at_mut(4);
                word1.swap_with_slice(word2);
            }
        }
    }

    // Compare the command's output with our manually swapped data
    let generated_swapped_data = fs::read(&generated_swapped_bin_path).unwrap();
    assert_eq!(generated_swapped_data, expected_swapped_data);

    // Test converting back
    let generated_png_path = get_asset_path("rgba32.word_swap.png");
    let mut cmd = Command::cargo_bin("pigment64").unwrap();
    cmd.args([
        "to-png",
        &generated_swapped_bin_path,
        "-o",
        &generated_png_path,
        "-f",
        "rgba32",
        "--width",
        &width.to_string(),
        "--height",
        &height.to_string(),
        "--word-swap",
    ])
    .assert()
    .success();

    // Convert the resulting PNG back to native format and compare the raw data
    let generated_png_file = fs::File::open(&generated_png_path).unwrap();
    let generated_png = PNGImage::read(generated_png_file).unwrap();
    let mut generated_unswapped_bin_data = Vec::new();
    generated_png
        .as_native(&mut generated_unswapped_bin_data, ImageType::Rgba32)
        .unwrap();

    let original_unswapped_bin_data = fs::read(original_unswapped_bin_path).unwrap();
    assert_eq!(generated_unswapped_bin_data, original_unswapped_bin_data);

    // Clean up generated files
    fs::remove_file(&generated_swapped_bin_path).unwrap();
    fs::remove_file(&generated_png_path).unwrap();
}
