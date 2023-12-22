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
    let mut bytes: Vec<u8> = Vec::new();

    let mut output = format!(
        "// {}\nstatic const unsigned char {}[{}] = \n{{\n\t",
        file_path,
        array_name,
        (width / 8) * height,
    );

    // Iterate over the pixels
    for (bit, pixel) in img.pixels().enumerate() {
        match pixel {
            // clear LSB
            Luma([0]) => byte &= 0xFE,
            // set LSB
            _ => byte |= 0x01,
        }

        // Store the byte after accumulating 8 bits
        if bit % 8 == 7 {
            bytes.push(byte);
            byte = 0;
        }

        // Rotate the byte to the left
        byte = byte.rotate_left(1);
    }

    // Insert commas and a new line after printing every 12 bytes
    let joined_bytes = format_bytes(bytes, 12);
    output.push_str(&joined_bytes);

    // Close the array
    output.push_str("\n};");

    Ok(output)
}

// Format bytes for C style array
fn format_bytes(data: Vec<u8>, items_per_line: usize) -> String {
    data.chunks(items_per_line)
        .map(|chunk| {
            let line: Vec<String> = chunk.iter().map(|item| format!("0x{:02x}", item)).collect();
            line.join(", ")
        })
        .collect::<Vec<String>>()
        .join(",\n\t")
}
