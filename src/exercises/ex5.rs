// https://adventofcode.com/2024/day/5
use crate::helper::tree::TreeNode;

pub fn solve_pt1() {
  let input = std::fs::read_to_string("inputs/5.txt").unwrap();
  let mut map = std::collections::HashMap
    ::<i32, std::collections::HashSet<i32>>::new();
  let mut middle_nums_sum = 0;
  for line in input.lines() {
    if let Some(pipe_index) = line.find('|') { 
      let before_num = line[..pipe_index].parse::<i32>().unwrap();
      let after_num = line[pipe_index + 1..].parse::<i32>().unwrap();
      if map.contains_key(&before_num) {
        map.get_mut(&before_num).unwrap().insert(after_num);
      }
      else {
        map.insert(before_num, std::collections::HashSet::from([after_num]));
      }
    }
    else if line.contains(',') {
      let nums = line.split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
      let mut correct_ordering = true;
      for i in 0..nums.len()-1 {
        let current_num = &nums[i];
        if map.contains_key(current_num) {
          if !map.get(current_num).unwrap().contains(&nums[i+1]) {
            correct_ordering = false;
            break;
          }
        }
      }
      if correct_ordering {
        println!("{} {}", nums[0], nums[nums.len() / 2]);
        middle_nums_sum += nums[nums.len() / 2];
      }
    }
  }
  println!("RESULT: {}", middle_nums_sum);
}

pub fn solve_pt2() {
  let input = std::fs::read_to_string("inputs/5.txt").unwrap();
  let mut tree = TreeNode::<i32>::new(0);
  let mut middle_nums_sum = 0;
  for line in input.lines() {
    if let Some(pipe_index) = line.find('|') { 
      let before_num = line[..pipe_index].parse::<i32>().unwrap();
      let after_num = line[pipe_index + 1..].parse::<i32>().unwrap();
      // if map.contains_key(&before_num) {
      //   map.get_mut(&before_num).unwrap().insert(after_num);
      // }
      // else {
      //   map.insert(before_num, std::collections::HashSet::from([after_num]));
      // }
    }
    else if line.contains(',') {
      let nums = line.split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
      let mut incorrect_ordering = true;
      for i in 0..nums.len()-1 {
        let current_num = &nums[i];
        // if map.contains_key(current_num) {
        //   if map.get(current_num).unwrap().contains(&nums[i+1]) {
        //     incorrect_ordering = false;
        //     break;
        //   }
        // }
      }
      if incorrect_ordering {
        for i in 0..nums.len()-1 {
          let current_num = &nums[i];
          // if map.contains_key(current_num) {
          //   map.get_mut(current_num).unwrap().remove(&nums[i+1]);
          // }
        }
        println!("{} {}", nums[0], nums[nums.len() / 2]);
        middle_nums_sum += nums[nums.len() / 2];
      }
    }
  }
  println!("RESULT: {}", middle_nums_sum);
}