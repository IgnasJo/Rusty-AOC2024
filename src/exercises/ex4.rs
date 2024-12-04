// https://adventofcode.com/2024/day/4

pub fn solve() {
  let input = std::fs::read_to_string("inputs/4.txt").unwrap();
  let mut char_matrix: Vec<Vec<char>> = Vec::new();
  char_matrix.append(&mut input.lines().map(|line| line.chars().collect()).collect());
  let mut count = 0;
  for row in 0..char_matrix.len() {
    let current_line = &char_matrix[row];
    for col in 0..current_line.len() {
      // left -> right
      if col < current_line.len() - 3 {
        if current_line[col] == 'X'
          && current_line[col + 1] == 'M'
          && current_line[col + 2] == 'A'
          && current_line[col + 3] == 'S' {
          count += 1;
        }
      }
      // right -> left
      if col > 2 {
        if current_line[col] == 'X'
          && current_line[col - 1] == 'M'
          && current_line[col - 2] == 'A'
          && current_line[col - 3] == 'S' {
          count += 1;
        }
      }
      // up -> down
      if row < char_matrix.len() - 3 {
        if char_matrix[row][col] == 'X'
          && char_matrix[row+1][col] == 'M'
          && char_matrix[row+2][col] == 'A'
          && char_matrix[row+3][col] == 'S' {
          count += 1;
        }
      }
      // down -> up
      if row > 2 {
        if char_matrix[row][col] == 'X'
          && char_matrix[row-1][col] == 'M'
          && char_matrix[row-2][col] == 'A'
          && char_matrix[row-3][col] == 'S' {
          count += 1;
        }
      }
      // right up diagonally
      if row < char_matrix.len() - 3 && col < current_line.len() - 3 {
        if char_matrix[row][col] == 'X'
          && char_matrix[row+1][col+1] == 'M'
          && char_matrix[row+2][col+2] == 'A'
          && char_matrix[row+3][col+3] == 'S' {
          count += 1;
        }
      }
      // right down diagonally
      if row > 2 && col < current_line.len() - 3 {
        if char_matrix[row][col] == 'X'
          && char_matrix[row-1][col+1] == 'M'
          && char_matrix[row-2][col+2] == 'A'
          && char_matrix[row-3][col+3] == 'S' {
          count += 1;
        }
      }
      // left up diagonally
      if row < char_matrix.len() - 3 && col > 2 {
        if char_matrix[row][col] == 'X'
          && char_matrix[row+1][col-1] == 'M'
          && char_matrix[row+2][col-2] == 'A'
          && char_matrix[row+3][col-3] == 'S' {
          count += 1;
        }
      }
      // left down diagonally
      if row > 2 && col > 2 {
        if char_matrix[row][col] == 'X'
          && char_matrix[row-1][col-1] == 'M'
          && char_matrix[row-2][col-2] == 'A'
          && char_matrix[row-3][col-3] == 'S' {
          count += 1;
        }
      }
    }
  }
  println!("RESULT: {}", count);
}