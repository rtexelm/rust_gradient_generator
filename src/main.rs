#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
// mod filters;
mod filter;
mod image;

enum EditOptions {
    AddNoise,
    Rotate,
    Resize,
    Crop,
    AddFilter,
}

enum Filter {
    Blur,
    EmphasizeBlue,
    Greenify,
    Warm,
}

fn greet_and_display_photos() -> Vec<String> {
    println!("Welcome to the photo edirtor!");
    println!("Here are the photos available to edit:");

    let mut photos = Vec::new();

    for entry in std::fs::read_dir("images").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            continue;
        }
        let filename: String = match path.file_name() {
            Some(filename) => filename.to_string_lossy().to_string(),
            None => continue,
        };
        photos.push(filename);
    }

    for (i, photo) in photos.iter().enumerate() {
        println!("{}: {}", i + 1, photo);
        println!("=====================");
    }
}

fn main() {}
