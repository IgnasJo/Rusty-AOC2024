// https://adventofcode.com/2024/day/1

pub fn solve() {
  let input = std::fs::read_to_string("inputs/1.txt").unwrap();
  // using RB or AVL trees would be better
  let mut left_list: Vec<i32> = Vec::new();
  let mut right_list: Vec<i32> = Vec::new();
  for line in input.lines() {
    let nums: Vec<i32> = line.split(' ').flat_map(str::parse::<i32>).collect();
    left_list.push(nums[0]);
    right_list.push(nums[1]);
  }
  let mut diff = 0;
  left_list.sort();
  right_list.sort();
  for i in 0..left_list.len() {
    diff += (left_list[i] - right_list[i]).abs();
  }
  println!("RESULT: {}", diff);
}