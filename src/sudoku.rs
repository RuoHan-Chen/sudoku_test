use std::fs::File;
use std::io::{self, BufReader, BufRead};

pub struct Puzzle {
    vals: [[u32; Puzzle::NUMROWS]; Puzzle::NUMROWS], 
}

impl Puzzle {
    const NUMROWS: usize = 9; 
    const SUBGRID_SIZE: usize = 3; 

    pub fn new() -> Self {
        Self {
            vals: [[0; Self::NUMROWS]; Self::NUMROWS],
        }
    }

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

    pub fn solve(&mut self) -> bool {
        return self.solve_index(0, 0); 
    }

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

    pub fn print(&self) {
        for row in &self.vals {
            println!("{:?}", row);
        }
    }

}