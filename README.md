# img-to-byte-array
This program converts a monochrome image to a byte array and generates a c header for embedded system projects. 

## Usage: 
```
cargo run path_to_image name_of_array > file_to_be_saved
```

## Example: 
```
cargo run sample/arrow_up.png arrow_up > sample/arrow_up.h
```
This reads "arrow_up.png" in "sample" folder and writes an array "arrow_up" in a file named "arrow_up.h".
