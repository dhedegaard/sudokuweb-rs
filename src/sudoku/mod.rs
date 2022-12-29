pub type Board = [[u8; 9]; 9];

pub fn solve(board: &Board) -> Option<Board> {
    solve_board(&mut board.clone())
}

fn solve_board(board: &mut Board) -> Option<Board> {
    let mut board = board.clone();
    let mut row = 0;
    let mut col = 0;
    let mut found = false;
    for i in 0..9 {
        for j in 0..9 {
            if board[i][j] == 0 {
                row = i;
                col = j;
                found = true;
                break;
            }
        }
        if found {
            break;
        }
    }
    if !found {
        return Some(board);
    }
    for num in 1..10 {
        if is_safe(&board, row, col, num) {
            board[row][col] = num;
            match solve_board(&mut board) {
                Some(board) => return Some(board),
                None => (),
            };
            board[row][col] = 0;
        }
    }
    None
}

fn is_safe(board: &Board, row: usize, col: usize, num: u8) -> bool {
    !used_in_row(board, row, num) && !used_in_col(board, col, num) && !used_in_box(board, row, col, num)
}

fn used_in_row(board: &Board, row: usize, num: u8) -> bool {
    for i in 0..9 {
        if board[row][i] == num {
            return true;
        }
    }
    false
}

fn used_in_col(board: &Board, col: usize, num: u8) -> bool {
    for i in 0..9 {
        if board[i][col] == num {
            return true;
        }
    }
    false
}

fn used_in_box(board: &Board, row: usize, col: usize, num: u8) -> bool {
    let row_start = row - row % 3;
    let col_start = col - col % 3;
    for i in 0..3 {
        for j in 0..3 {
            if board[row_start + i][col_start + j] == num {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let input: Board = [
        [0, 0, 2, 0, 3, 0, 7, 4, 0],
        [6, 0, 5, 0, 9, 0, 8, 0, 0],
        [4, 0, 0, 2, 0, 0, 0, 6, 9],
        [8, 0, 9, 5, 0, 0, 1, 0, 6],
        [0, 1, 0, 0, 0, 0, 0, 7, 0],
        [7, 0, 4, 0, 0, 3, 9, 0, 8],
        [3, 7, 0, 0, 0, 9, 0, 0, 2],
        [0, 0, 1, 0, 6, 0, 4, 0, 7],
        [0, 4, 6, 0, 7, 0, 5, 0, 0],
    ];
    let expected_result: Board = [
        [1, 9, 2, 6, 3, 8, 7, 4, 5],
        [6, 3, 5, 7, 9, 4, 8, 2, 1],
        [4, 8, 7, 2, 1, 5, 3, 6, 9],
        [8, 2, 9, 5, 4, 7, 1, 3, 6],
        [5, 1, 3, 9, 8, 6, 2, 7, 4],
        [7, 6, 4, 1, 2, 3, 9, 5, 8],
        [3, 7, 8, 4, 5, 9, 6, 1, 2],
        [9, 5, 1, 3, 6, 2, 4, 8, 7],
        [2, 4, 6, 8, 7, 1, 5, 9, 3],
    ];

    let result = solve(&input);

    assert!(result.is_some());
    let result = result.unwrap();

    println!("{:?}", result);
    assert_eq!(
        result,
        expected_result
    );
  }
}