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

pub fn solve_pt2() {
  let input = std::fs::read_to_string("inputs/1.txt").unwrap();
  let mut map = std::collections::HashMap::new();
  for line in input.lines() {
    let nums: Vec<i32> = line.split(' ').flat_map(str::parse::<i32>).collect();
    if map.contains_key(&nums[0]) {
      let vals: (bool, i32) = *map.get(&nums[0]).unwrap();
      let left_exists = vals.0;
      if !left_exists {
        map.insert(nums[0], (true, vals.1));
      }
    }
    else {
      map.insert(nums[0], (true, 0));
    }
    if map.contains_key(&nums[1]) {
      let vals = *map.get(&nums[1]).unwrap();
      map.insert(nums[1], (vals.0, vals.1 + 1));
    }
    else {
      map.insert(nums[1], (false, 1));
    }
  }
  let mut similiarity = 0;
  for (key, value) in map.iter() {
    println!("key: {}, left_occurences: {}, right_occurences: {}", key, value.0, value.1);
    if value.0 {
      similiarity += value.1 * key;
    }
  }
  println!("RESULT: {}", similiarity);
}