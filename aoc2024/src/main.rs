use std::fs::read_to_string;

fn main() {
    second();
}

fn second() {
    fn is_unsafe(line: &str) -> bool {
        let nums = line.trim().split(' ').flat_map(str::parse::<i32>).collect::<Vec<_>>();
        let mut prev_num: i32 = nums[0];
        let current_num: i32 = nums[1];
        let is_increasing = current_num > prev_num;
        for current_num in &nums[1..] {
            let diff = (*current_num - prev_num).abs();
            if (*current_num < prev_num && is_increasing)
            || (*current_num > prev_num && !is_increasing)
            || (diff < 1 || diff > 3) {
                return false;
            }
            prev_num = *current_num;
        }
        true
    }
    const FILENAME: &str = "inputs/2.txt";
    let mut unsafe_count: i32 = 0;
    for line in read_to_string(FILENAME).unwrap().lines() {
        if is_unsafe(line) {
            unsafe_count += 1;
        }
    }
    println!("{}", unsafe_count);
}