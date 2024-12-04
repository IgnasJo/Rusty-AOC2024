// https://adventofcode.com/2024/day/3
use regex::Regex;

pub fn solve() {
  let input = std::fs::read_to_string("inputs/3.txt").unwrap();
  let regex = r"mul\((?<num1>\d+),(?<num2>\d+)\)";
  let re = Regex::new(&regex).unwrap();
  println!("{}", input);
  let mut results = vec![];
  for (_, [num1, num2]) in re.captures_iter(&input).map(|c| c.extract()) {
    results.push((num1.parse::<u64>(), num2.parse::<u64>()));
  }
  let mut sum: u64 = 0;
  for ele in results {
    match (ele.0, ele.1) {
      (Ok(num1), Ok(num2)) => sum += num1 * num2,
      (Err(err), _) => println!("Error parsing num1: {}", err),
      (_, Err(err)) => println!("Error parsing num2: {}", err),
    }
  }
  println!("RESULT: {}", sum);
}