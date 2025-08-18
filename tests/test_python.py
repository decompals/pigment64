import os
import pytest
import pigment64

# Get the directory of the current test file
TESTS_DIR = os.path.dirname(os.path.abspath(__file__))

def test_extract_palette_from_png_bytes():
    """
    Tests that a valid CI4 PNG with a TLUT returns the expected palette.
    """
    # Construct the full path to the test asset files
    png_path = os.path.join(TESTS_DIR, "ci4.png")
    expected_palette_path = os.path.join(TESTS_DIR, "ci4.tlut.bin")

    # Check that test files exist before trying to open them
    if not os.path.exists(png_path):
        pytest.fail(f"Test asset not found: {png_path}")
    if not os.path.exists(expected_palette_path):
        pytest.fail(f"Expected result file not found: {expected_palette_path}")

    with open(png_path, "rb") as f:
        png_binary_data = f.read()

    with open(expected_palette_path, "rb") as f:
        expected_palette_bytes = f.read()

    # Call the function from the Rust library
    actual_palette_bytes = pigment64.extract_palette_from_png_bytes(png_binary_data)

    # Check that the actual output matches the expected output
    assert actual_palette_bytes == expected_palette_bytes


def test_roundtrip_rgba16():
    """
    Performs a round-trip test for the RGBA16 format.
    1. Reads a native RGBA16 .bin file.
    2. Converts it to PNG bytes using the rust library.
    3. Creates a PNGImage from the PNG bytes.
    4. Converts it back to native RGBA16 bytes.
    5. Asserts the result is identical to the original.
    """
    # Arrange to read the original native binary data
    native_bin_path = os.path.join(TESTS_DIR, "rgba16.png.bin")
    with open(native_bin_path, "rb") as f:
        original_native_bytes = f.read()

    # Convert native binary to PNG bytes
    png_bytes = pigment64.native_to_png(
        original_native_bytes, "rgba16", 256, 256, None
    )

    # Convert the PNG bytes back to native RGBA16
    png_image = pigment64.PNGImage(png_bytes)
    roundtrip_native_bytes = png_image.as_rgba16()

    # Assert
    assert roundtrip_native_bytes == original_native_bytes
