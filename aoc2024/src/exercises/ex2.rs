use std::fs::read_to_string;

pub fn solve() {
  const FILENAME: &str = "inputs/2_test.txt";
  let mut unsafe_count: i32 = 0;
  for line in read_to_string(FILENAME).unwrap().lines() {
      let nums = line.trim().split(' ').flat_map(str::parse::<i32>).collect::<Vec<_>>();
      if are_levels_unsafe_pt2(nums) {
          unsafe_count += 1;
      }
  }
  println!("{}", unsafe_count);
}

fn are_levels_unsafe_pt2(levels: Vec<i32>) -> bool {
  let to_index = levels.len()-1;
  for i in 0..to_index {
      if !are_levels_unsafe_pt1(levels[i..].to_vec()) {
          println!("UNSAFE");
      }
  }
  true
}

fn are_adjacent_levels_unsafe(previous_level: i32, current_level: i32, is_increasing: bool) -> bool {
  let diff = (current_level - previous_level).abs();
  (current_level < previous_level && is_increasing)
      || (current_level > previous_level && !is_increasing)
      || (diff < 1 || diff > 3)
}

fn are_levels_unsafe_pt1(levels: Vec<i32>) -> bool {
  let mut previous_level: i32 = levels[0];
  let current_level: i32 = levels[1];
  let is_increasing = current_level > previous_level;
  for current_level in &levels[1..] {
      println!("{} {}", previous_level, *current_level);
      if are_adjacent_levels_unsafe(previous_level, *current_level, is_increasing) {
          return false;
      }
      previous_level = *current_level;
  }
  true
}