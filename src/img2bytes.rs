//! # LCD bitmap
//!
//! Usage: cargo run path_to_image name_of_array > file_to_be_saved
//! Example: cargo run sample/arrow_up.png arrow_up > sample/arrow_up.h

use image::Luma;
use std::error::Error;

pub fn convert(file_path: &str, array_name: &str) -> Result<String, Box<dyn Error>> {
    // Open the image
    let img = image::open(file_path)?.to_luma8();
    let (width, height) = img.dimensions();

    let mut byte: u8 = 0;

    let mut output = format!(
        "static const unsigned char {}[{}] = \n{{ // {}",
        array_name,
        (width / 8) * height,
        file_path
    );

    // Iterate over the pixels
    for (bit, pixel) in img.pixels().enumerate() {
        match pixel {
            // clear LSB
            Luma([0]) => byte &= 0xFE,
            // set LSB
            _ => byte |= 0x01,
        }

        // Print after accumulating 8 bits
        if bit % 8 == 7 {
            output.push_str(&format!("0x{:02x},", byte));
            byte = 0;
        }

        // Insert a new line after printing every 12 bytes
        if bit % (12 * 8) == 0 {
            output.push_str("\n\t")
        }

        // Rotate the byte to the left
        byte = byte.rotate_left(1);
    }

    Ok(format!("{}\r\n}};", output))
}
