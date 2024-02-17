use std::io;
use std::fs;
fn main() {
    let file_path = "/Users/gabriel/code/rust/text-finder-tool/test-data/text.txt";
    let buffer = match fs::read_to_string(file_path){
        Ok(content) => {
            println!("Loaded file into buffer");
            content
        },
        Err(_) => {
            println!("Failed to load file into buffer");
            String::new()
        }
    };

    loop {
        println!("Enter the text you want to find: ");
        let mut text_to_find: String = String::new();

        match io::stdin().read_line(&mut text_to_find){
            Ok(_) => println!("Finding {}", text_to_find),
            Err(_) => {
                print!("Failed to start finding {}", text_to_find);
                continue;
            }
        }

        if buffer.contains(&text_to_find) {
            println!("Found!");
        } else {
            println!("Not found!");
        }

        break;
    }
}
