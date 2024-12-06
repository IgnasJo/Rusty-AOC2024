// https://adventofcode.com/2024/day/6

pub fn solve() {
  let input = std::fs::read_to_string("inputs/6.txt").unwrap();
  let mut route_matrix = Vec::<Vec::<u8>>::new();
  let mut start_position = Position {col: 0, row: 0};
  // READ
  for (row, line) in input.lines().enumerate() {
    route_matrix.push(Vec::<u8>::new());
    for col in 0..line.len() {
      if AVAILABLE_DIRECTIONS.contains(&line.as_bytes()[col]) {
        start_position = Position {col, row};
      }
      route_matrix[row].push(line.as_bytes()[col]);
    }
  }
  // SOLVE
  let mut route_navigator = RouteNavigator::new(&mut route_matrix, start_position);
  route_navigator.traverse();
  route_navigator.print_results();
}

const AVAILABLE_DIRECTIONS: [u8; 4] = [b'^', b'v', b'>', b'<'];
struct Position {
  col: usize,
  row: usize
}

struct RouteNavigator<'a> {
  route: &'a mut Vec<Vec<u8>>,
  pos: Position,
  state: u8,
  unique_visit_count: u32
}

impl<'a> RouteNavigator<'a> {
  fn new(route: &'a mut Vec<Vec<u8>>, pos: Position) -> Self {
    let state = route[pos.row][pos.col];
    RouteNavigator {
      route,
      pos,
      state,
      unique_visit_count: 0
    }
  }

  fn is_current_position_on_bounds(&self) -> bool {
    let Position {col, row} = self.pos;
    row == 0 || row >= self.route.len() || col == 0 || col >= self.route[row].len() 
  }

  fn traverse(&mut self) {
    // mark starting pos
    self.mark_used_path();
    loop {
      self.move_next();
      if self.is_current_position_on_bounds() {
        break;
      }
    }  
  }

  fn move_next(&mut self) {
    // first move forward
    let mut next_pos = self.peek_forward();
    let next_cell = self.route[next_pos.row][next_pos.col];
    if next_cell == b'#' {
      self.rotate_clockwise();
      next_pos = self.peek_forward();
    }
    self.pos = next_pos;
    // then mark the path
    let current_cell = self.route[self.pos.row][self.pos.col];
    if current_cell == b'.' {
      self.mark_used_path();
    }
  }

  fn print_results(&self) {
    for row in &*self.route {
      println!("{}", row.iter().map(|c| *c as char).collect::<String>());
    }
    println!("Position: column: {}, row: {}", self.pos.col, self.pos.row);
    println!("RESULT: {}", self.unique_visit_count);
  }

  fn peek_forward(&self) -> Position {
    match self.state {
      b'^' => Position {col: self.pos.col, row: self.pos.row - 1},
      b'v' => Position {col: self.pos.col, row: self.pos.row + 1},
      b'>' => Position {col: self.pos.col + 1, row: self.pos.row},
      b'<' => Position {col: self.pos.col - 1, row: self.pos.row},
      _ => std::unreachable!()
    }
  }
  
  fn mark_used_path(&mut self) {
    let current_cell = &mut self.route[self.pos.row][self.pos.col];
    *current_cell = b'X';
    self.unique_visit_count += 1;
  }

  fn rotate_clockwise(&mut self) {
    self.state = match self.state {
      b'^' => b'>',
      b'>' => b'v',
      b'v' => b'<',
      b'<' => b'^',
      _ => self.state
    }
  }
}