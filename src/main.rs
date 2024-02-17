use std::io::{self, BufRead};
use std::fs;
use std::path;
fn main() {
    let file_path = path::Path::new("/Users/gabriel/code/rust/text-finder-tool/test-data/text.txt");

    loop {
        // Take file path and open file if valid
        let file = match fs::File::open(&file_path) {
            Ok(file) => file,
            Err(e) => {
                println!("Invalid file or file path.");
                continue;
            }
        };
        let reader = io::BufReader::new(file);

        // take string to find and search
        println!("Enter the text you want to find: ");
        let mut text_to_find: String = String::new();

        match io::stdin().read_line(&mut text_to_find){
            Ok(_) => println!("{}", text_to_find),
            Err(_) => continue,
        }

        for line in reader.lines() {
            match line {
                Ok(line) => println!("{}", line),
                Err(e) => eprintln!("Failed to read line: {}", e),
            }
        }
    }
}
