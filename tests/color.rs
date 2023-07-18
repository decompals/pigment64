use pigment64::color::Color;

#[test]
fn test_color_new() {
    // Test case 1: Color with alpha component
    let color = Color::RGBA(255, 128, 64, 255);
    assert_eq!(color.r, 255);
    assert_eq!(color.g, 128);
    assert_eq!(color.b, 64);
    assert_eq!(color.a, 255);

    // Test case 2: Color without alpha component
    let color = Color::RGB(255, 128, 64);
    assert_eq!(color.r, 255);
    assert_eq!(color.g, 128);
    assert_eq!(color.b, 64);
    assert_eq!(color.a, 255);
}

#[test]
fn test_color_from_u16() {
    // Test case 1: Pixel value with maximum component values
    let pixel: u16 = 0xFFFF; // Binary: 1111111111111111
    let color = Color::WHITE;
    assert_eq!(Color::from_u16(pixel), color);

    // Test case 2: Pixel value with minimum component values
    let pixel: u16 = 0x0000; // Binary: 0000000000000000
    let color = Color::TRANSPARENT;
    assert_eq!(Color::from_u16(pixel), color);

    // Test case 3: Random pixel value
    let pixel: u16 = 0b1101010101010101; // Binary: 11010 10101 01010 101
    let color = Color::RGBA(214, 173, 82, 255);
    assert_eq!(Color::from_u16(pixel), color);
}

#[test]
fn test_color_to_u16() {
    // Test case 1: Pixel value with maximum component values
    let color = Color::WHITE;
    let pixel = color.to_u16();
    assert_eq!(pixel, 0xFFFF);

    // Test case 2: Pixel value with minimum component values
    let color = Color::BLACK;
    let pixel = color.to_u16();
    assert_eq!(pixel, 0x0001);

    // Test case 3: Random pixel value
    let color = Color::RGBA(213, 172, 82, 255);
    let pixel = color.to_u16();
    assert_eq!(pixel, 0b1101010101010101);
}

#[test]
fn test_color_rba16() {
    // Test case 1: Color value with maximum component values
    let color = Color::WHITE;
    let pixel = color.rgba16();
    assert_eq!(pixel, [0xFF, 0xFF]);

    // Test case 2: Color value with minimum component values
    let color = Color::BLACK;
    let pixel = color.rgba16();
    assert_eq!(pixel, [0, 1]);

    // Test case 3: Random color values
    let color = Color::RGBA(120, 200, 50, 150);
    let pixel = color.rgba16();
    assert_eq!(pixel, [126, 76]);
}

#[test]
fn test_color_rgb_to_intensity() {
    // Test case 1: Pixel value with maximum component values
    let color = Color::WHITE;
    let intensity = color.rgb_to_intensity();
    assert_eq!(intensity, 255);

    // Test case 2: Pixel value with minimum component values
    let color = Color::BLACK;
    let intensity = color.rgb_to_intensity();
    assert_eq!(intensity, 0);

    // Test case 3: RGB values with equal intensity
    let color = Color::RGBA(128, 128, 128, 255);
    let intensity = color.rgb_to_intensity();
    assert_eq!(intensity, 128);

    // Test case 4: RGB values with different intensities
    let color = Color::RGBA(255, 128, 64, 255);
    let intensity = color.rgb_to_intensity();
    assert_eq!(intensity, 150);
}
