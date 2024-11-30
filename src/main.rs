use std::io::{self, Write}; 

fn main() {
    print!("Enter path to puzzle file: "); 
    io::stdout().flush().unwrap(); 

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();

    let defaultFile = "sudoku-test1.txt"; 
    let fileSuffix = if input.trim().is_empty() {
        defaultFile
    } else {
        input.trim() 
    }; 
    let filename = &("../txt/".to_owned() + fileSuffix); 

    println!("Using file: {}", filename); 
}
