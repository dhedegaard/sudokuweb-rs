mod utils;
mod sudoku;

use sudoku::{Board, solve};

use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, sudokuweb-rs!");
}

#[wasm_bindgen]
pub fn solve_board(board: String) -> Option<String> {
    // TODO: Parse the board as JSON and validate the input.
    let board: Board = serde_json::from_str(&board).unwrap();

    // TODO: Solve the board.
    let result = solve(&board);

    // TODO: Serialize the result.
    match result {
        Some(result) => {
            let result = serde_json::to_string(&result).unwrap();
            Some(result)
        },
        None => None,
    }
}
