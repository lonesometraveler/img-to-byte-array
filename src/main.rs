use std::env;
use std::process;

use img2bytes::ImgToBytes;

fn main() {
    let writer = ImgToBytes::new(env::args()).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments: {}", err);
        process::exit(1);
    });

    let output = writer.run();
    println!("{}", output);
}
