use std::fs::File;
use std::io::{self, BufReader, BufRead};

pub struct Puzzle {
    vals: [[u32; Puzzle::NUMROWS]; Puzzle::NUMROWS], 
}

impl Puzzle {
    const NUMROWS: usize = 9; 
    const SUBGRID_SIZE: usize = 3; 

    /**
     * Constructor for a Puzzle object, which contains two constants and an array of values
     * 
     */
    pub fn new() -> Self {
        Self {
            vals: [[0; Self::NUMROWS]; Self::NUMROWS],
        }
    }

    /**
     * Loads in a sudoku puzzle from a given file path
     * 
     * @param  path An absolute or relative path to the file containing the sudoku puzzle
     * @return      Success status of the function (whether the method ran successfully)
     */
    pub fn load_from_file(&mut self, path: &String) -> io::Result<()> {
        let f = File::open(path)?;
        let mut reader = BufReader::new(f);

        for i in 0..Self::NUMROWS {
            let mut input = String::new(); 
            reader.read_line(&mut input)?; 
            
            let mut j = 0; 
            for ch in input.chars() {
                if let Some(digit) = ch.to_digit(10) { // store all digits in row to array
                    self.vals[i][j] = digit; 
                    j += 1; 
                } 
            }
        }

        Ok(())
    }

    /**
     * Check whether a given value can be placed at a given position in the board
     * 
     * @param  row      Row index of the position where the value will be placed
     * @param  col      Column index of the positoin where the value will be placed
     * @param  test_val Value to be tested
     * @return          Whether test_val can be placed at the given position  
     */
    fn check_valid(&self, row: usize, col: usize, test_val: u32) -> bool {
        // check rows and cols
        for i in 0..Self::NUMROWS {
            if self.vals[row][i] == test_val { return false; }
            if self.vals[i][col] == test_val { return false; }
        }

        // use integer division to round down
        let subgrid_start_row: usize = Self::SUBGRID_SIZE * (row / Self::SUBGRID_SIZE); 
        let subgrid_start_col: usize = Self::SUBGRID_SIZE * (col / Self::SUBGRID_SIZE); 
        
        // check subgrid
        for i in 0..Self::SUBGRID_SIZE {
            for j in 0..Self::SUBGRID_SIZE {
                let curr_row: usize = subgrid_start_row + i; 
                let curr_col: usize = subgrid_start_col + j; 
                if self.vals[curr_row][curr_col] == test_val { return false; }
            }
        }
        
        return true; 
    }

    /**
     * Solves the currently loaded sudoku board
     * 
     * @return Whether the sudoku board can be solved 
     */
    pub fn solve(&mut self) -> bool {
        return self.solve_index(0, 0); 
    }

    /**
     * Solves the currently loaded sudoku board starting from the given position
     * 
     * @param  row Row index of the current position on the board  
     * @param  col Column index of the current position on the board
     * @return     Whether the board can be solved starting from the given position
     */
    fn solve_index(&mut self, row: usize, col: usize) -> bool {
        if row == Self::NUMROWS {
            return true; 
        }
        
        if col == Self::NUMROWS {
            return self.solve_index(row+1, 0); 
        }

        if self.vals[row][col] != 0 {
            return self.solve_index(row, col+1); 
        }

        let mut solved:bool = false; 
        let mut test_val: u32 = 1; 
        while test_val <= Self::NUMROWS.try_into().unwrap() && !solved {
            if self.check_valid(row, col, test_val) {
                self.vals[row][col] = test_val; 
                solved = self.solve_index(row, col+1); 
                if !solved {
                    self.vals[row][col] = 0; 
                }
            }

            test_val += 1; 
        }

        return solved; 
    }

    /**
     * Returns a formatted string of a 3-value subset within the board
     * 
     * @param  row       Row index for the 3-value subset
     * @param  col_start Column index of the first value in the subset
     * @return           3 values at given row starting at col_start, separated by spaces
     */
    fn print_subrow(&self, row: usize, col_start: usize) -> String {
        let first_val: String = self.vals[row][col_start].to_string(); 
        let second_val: String = self.vals[row][col_start+1].to_string(); 
        let third_val: String = self.vals[row][col_start+2].to_string(); 
        let subrow_formatted: String = format!("{} {} {}", first_val, second_val, third_val); 
        return subrow_formatted; 
    }

    /**
     * Returns a formatted string of a row on the board 
     * 
     * @param  row Row of the puzzle to be converted to a string
     * @return     Row of the puzzle, demarcated with | to indicate subgrids 
     */
    fn print_row(&self, row: usize) -> String {
        let first_subsec: String = self.print_subrow(row, 0); 
        let second_subsec: String = self.print_subrow(row, 3); 
        let third_subsec: String = self.print_subrow(row, 6); 
        let row_formatted = format!("| {} | {} | {} |", first_subsec, second_subsec, third_subsec);
        return row_formatted; 
    }

    /**
     * Prints the entire board 
     */
    pub fn print(&self) {
        println!("+-------+-------+-------+"); 
        for i in 0..3 {
            println!("{}", self.print_row(i));
        }
        println!("+-------+-------+-------+"); 
        for i in 3..6 {
            println!("{}", self.print_row(i));
        }
        println!("+-------+-------+-------+"); 
        for i in 6..Self::NUMROWS {
            println!("{}", self.print_row(i));
        }
        println!("+-------+-------+-------+"); 
    }
/**
 * Checks if two puzzles are identical used for testing
 * 
 * @param  path Path to the other puzzle file
 * @return      Whether the two puzzles are the same
 */
pub fn equals(&self, path: &String) -> io::Result<bool> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);

    for (i, line) in reader.lines().enumerate() {
        let line = line?; // Read the line
        let nums: Vec<u32> = line
            .split_whitespace() // Split the line into numbers by spaces
            .filter_map(|num| num.parse::<u32>().ok()) // Parse each number
            .collect();

        // Compare the numbers with the corresponding row in the current puzzle
        if nums.len() != Self::NUMROWS || nums != self.vals[i] {
            return Ok(false); // If any row differs, return false
        }
    }

    Ok(true) // If all rows are identical, return true
}



}