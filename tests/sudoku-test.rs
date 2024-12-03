use project4_29::Puzzle; 

//Test cases
#[test]
fn test_valid_puzzle1() {
    test_puzzle(
        "txt/sudoku-test1.txt",
        "txt/sudoku-test1-solution.txt",
        true,
    );
}

#[test]
fn test_valid_puzzle2() {
    test_puzzle(
        "txt/sudoku-test2.txt",
        "txt/sudoku-test2-solution.txt",
        true,
    );
}

#[test]
fn test_unsolvable_puzzle() {
    test_puzzle("txt/sudoku-impossible.txt", "", false);
}

/// Helper function to test puzzles
fn test_puzzle(puzzle_file: &str, solution_file: &str, should_be_solvable: bool) {
    let mut puzzle = Puzzle::new();

    let load_result = puzzle.load_from_file(&puzzle_file.to_string());
    assert!(load_result.is_ok(), "Failed to load puzzle from file: {}", puzzle_file);

    println!("Testing unsolved puzzle from: {}", puzzle_file);
    puzzle.print();

    let solve_status = puzzle.solve();
    assert_eq!(
        solve_status, should_be_solvable,
        "Puzzle solvability mismatch for: {}",
        puzzle_file
    );

    if should_be_solvable {
        println!("Testing solved puzzle:");
        puzzle.print();

        if !solution_file.is_empty() {
            match puzzle.equals(&solution_file.to_string()) {
                Ok(true) => println!("The solved puzzle matches the expected solution!"),
                Ok(false) => panic!("The solved puzzle does not match the expected solution."),
                Err(e) => panic!("Error while comparing puzzles: {}", e),
            }
        }
    }
}
