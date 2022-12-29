mod utils;
mod sudoku;

use sudoku::{Board, solve};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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
