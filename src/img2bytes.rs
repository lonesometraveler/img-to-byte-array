//! # LCD bitmap
//!
//! Usage: cargo run path_to_image name_of_array > file_to_be_saved
//! Example: cargo run sample/arrow_up.png arrow_up > sample/arrow_up.h

use image::Luma;
use std::path::Path;

pub struct ImgToBytes {
    file: String,
    array_name: String,
}

impl ImgToBytes {
    pub fn new(mut args: std::env::Args) -> Result<ImgToBytes, &'static str> {
        args.next(); // skip the first argument which is the name of the program

        let file = match args.next() {
            Some(arg) => arg,
            None => return Err("no file specified. Usage: cargo run path_to_image_folder name_of_array > filename_to_be_saved.h"),
        };

        let array_name = match args.next() {
            Some(arg) => arg,
            None => return Err("no array name specified. Usage: cargo run path_to_image_folder name_of_array > filename_to_be_saved.h")
        };

        Ok(ImgToBytes { file, array_name })
    }

    pub fn run(&self) -> String {
        match print_array(Path::new(&self.file), &self.array_name) {
            Ok(f) => f,
            _ => String::from("error"),
        }
    }
}

fn print_array(path: &Path, array_name: &str) -> Result<String, &'static str> {
    // TODO: error handling

    let img = image::open(path).unwrap().to_luma();
    let (width, height) = img.dimensions();

    let mut byte: u8 = 0;

    let mut output = format!(
        "static const unsigned char {}[{}] = \r\n{{ // {}",
        array_name,
        (width / 8) * height,
        path.to_str().unwrap()
    );

    for (bit, pixel) in img.pixels().enumerate() {
        match pixel {
            Luma([0]) => byte &= 0xFE,
            _ => byte |= 0x01,
        }

        if bit % 8 == 7 {
            output = format!("{}0x{:02x},", output, byte);
            byte = 0;
        }

        if bit % (12 * 8) == 0 {
            output.push_str("\n\t")
        }

        byte = byte.rotate_left(1);
    }

    Ok(format!("{}\r\n}};", output))
}
