use std::fs;
use sudoku::Puzzle;
#[test]
fn test_sudoku_solver() {
    let mut puzzle = Puzzle::new();

    let puzzle_file = "txt/sudoku-test1.txt"; 
    let solution_file = "txt/sudoku-test1-solution.txt"; 

    let load_result = puzzle.load_from_file(&puzzle_file.to_string());
    assert!(load_result.is_ok(), "Failed to load puzzle from file");

    println!("Testing unsolved puzzle:");
    puzzle.print();

    let solve_status = puzzle.solve();
    assert!(solve_status, "The puzzle should be solvable");

    println!("Testing solved puzzle:");
    puzzle.print();

    let expected_solution = fs::read_to_string(solution_file)
        .expect("Failed to read the solution file")
        .replace("\r\n", "\n"); 

    let actual_solution = puzzle_to_string(&puzzle); 
    assert_eq!(actual_solution, expected_solution, "The solution does not match the expected result");
}

fn puzzle_to_string(puzzle: &Puzzle) -> String {
    let mut result = String::new();
    for i in 0..Puzzle::NUMROWS {
        for j in 0..Puzzle::NUMROWS {
            result.push_str(&puzzle.vals[i][j].to_string());
            if j < Puzzle::NUMROWS - 1 {
                result.push(' '); 
            }
        }
        if i < Puzzle::NUMROWS - 1 {
            result.push('\n'); 
        }
    }
    result
}
