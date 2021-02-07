use std::fs;
use std::cmp;

fn process(mass: i32) -> i32 {
    if mass <= 0 {
        return 0;
    }
    let fuel = (mass / 3) - 2;
    cmp::max(0, fuel + process(fuel))
}

fn main() {
    assert!(process(14) == 2);
    assert!(process(100756) == 50346);
    let contents = fs::read_to_string("input.txt")
        .expect("File reading failed");
    let iter = contents.split_whitespace();
    let mut sum = 0;
    for num in iter {
        let value: i32 = num.parse().unwrap();
        sum += process(value);
    }
    println!("Hello, world! {}", sum);
}
