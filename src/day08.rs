use std::fs;
use std::collections::HashMap;


fn collect_counts(contents: &str) -> HashMap<char, i32> {
    let mut map = HashMap::new();
    for ch in contents.chars() {
        map.entry(ch)
            .and_modify(|e| { *e += 1 })
            .or_insert(1);
    }
    map
}

fn split_units(contents: &str, size: usize) -> Vec<&str> {
    let num_units = contents.len() / size;
    let mut result: Vec<&str> = Vec::new();
    let mut remaining: &str = contents.clone();
    for _i in 0..num_units {
        let (first, last) = remaining.split_at(size);
        result.push(first);
        remaining = last;
    }
    result
}

fn decode(layers: &Vec<Vec<&str>>, h: usize, w: usize) -> char {
    for layer in layers {
        // layer should be a Vec<&str>
        let value: char = layer[h].as_bytes()[w] as char;
        if value == '1' {
            return value;
        } else if value == '0' {
            return ' ';
        }
    }
    return '0';
}

fn main() {
    println!("{:?}", split_units("111222333", 3));
    assert!(split_units("111222333", 3) == vec!["111", "222", "333"]);
    let contents = fs::read_to_string("input.txt")
        .expect("File reading failed");
    let height = 6;
    let width = 25;
    let layers = split_units(contents.trim(), height * width).iter().map(|x| split_units(x, width)).collect();
    for h in 0..height {
        for w in 0..width {
            print!("{}", decode(&layers, h, w));
        }
        println!();
    }
}
