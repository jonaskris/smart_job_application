use rfd::*;
use std::fs;

mod process;

fn main() {
    // Read file
    let original_file_path = FileDialog::new()
        .add_filter("text", &["txt"])
        .pick_file();

    let original_contents = match original_file_path {
        Some(path) => fs::read_to_string(path).unwrap(),
        None => {
            println!("Dident pick a file, terminating process...");
            std::process::exit(0);
        }
    };

    // Process file
    let processed_contents = process::process(&original_contents);


    // Write file
    loop {
        let processed_file_path = FileDialog::new()
            .save_file();

        let processed_file_path = match processed_file_path {
            Some(path) => path,
            None => {
                println!("Please choose a save location for the processed file!");
                continue;
            }
        };

        fs::write(processed_file_path, &processed_contents).unwrap();
        break;
    }
}