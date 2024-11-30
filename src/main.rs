mod sudoku; 

use std::io::{self, Write}; 
use sudoku::Puzzle; 

fn main() {
    let mut puzzle = Puzzle::new(); 

    print!("Enter path to puzzle file: "); 
    io::stdout().flush().unwrap(); 

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();

    let default_file = "sudoku-test1.txt"; 
    let file_suffix = if input.trim().is_empty() {
        default_file
    } else {
        input.trim() 
    }; 
    let filename = &("txt/".to_owned() + file_suffix); 

    println!("Using file: {}", filename); 

    let _ = puzzle.load_from_file(filename);
    println!("Unsolved puzzle:"); 
    puzzle.print(); 

    let solve_status = puzzle.solve(); 

    if solve_status {
        println!("Puzzle can be solved!"); 
        puzzle.print(); 
    } else {
        println!("Puzzle is impossible"); 
    }
}
