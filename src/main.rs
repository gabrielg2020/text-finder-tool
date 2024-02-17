use std::{error, io};
fn main() {
    let file_path: &str = "/Users/gabriel/code/rust/text-finder-tool/test-data/text.txt";

    loop {
        println!("Enter the text you want to find: ");
        let mut text_to_find: String = String::new();

        match io::stdin().read_line(&mut text_to_find){
            Ok(_) => println!("{}", text_to_find),
            Err(_) => continue,
        };
    }
    
}
