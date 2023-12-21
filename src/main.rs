mod img2bytes;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let src_file_path = args
        .get(1)
        .expect("Usage: cargo run path_to_image_folder name_of_array > filename_to_be_saved.h");
    let array_name = args
        .get(2)
        .expect("Usage: cargo run path_to_image_folder name_of_array > filename_to_be_saved.h");

    match img2bytes::convert(src_file_path, array_name) {
        // Write to file
        Ok(result) => println!("{}", result),
        Err(err) => eprintln!("Error: {}", err),
    }
}
