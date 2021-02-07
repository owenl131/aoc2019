// #![feature(map_first_last)]
use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Ordering::Equal;
use std::cmp::Reverse;

fn viewable_200(map: &Vec<(i32, i32)>, x: i32, y: i32) -> (i32, i32) {
    // println!("{:?}", map);
    let mut seendirs = HashSet::new();
    let mut dirs: Vec<(f32, i32, i32)> = Vec::new();
    let mut sets: HashMap<(i32, i32), BinaryHeap<Reverse<i32>>> = HashMap::new();
    for (ax, ay) in map {
        let dy = ay - y;
        let dx = ax - x;
        let mut angle = (dx as f32).atan2(-dy as f32);
        if angle < 0.0 {
            angle += std::f32::consts::PI * 2.0;
        }
        let g = num_integer::gcd(dy, dx);
        if g == 0 {
            continue;
        }
        let dir = (dx / g, dy / g);
        if !seendirs.contains(&dir) {
            seendirs.insert(dir);
            dirs.push((angle, dx / g, dy / g));
            let opt = sets.insert(dir, BinaryHeap::new());
            assert!(opt.is_none());
        }
        sets.get_mut(&dir).unwrap().push(Reverse(g));
    }
    dirs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));
    // println!("{:?}", dirs);
    let mut index = 0;
    let mut last_x = 0;
    let mut last_y = 0;
    for _i in 0..200 {
        let mut dir = (dirs[index].1, dirs[index].2);
        while sets.get(&dir).unwrap().len() == 0 {
            index += 1;
            index %= dirs.len();
            dir = (dirs[index].1, dirs[index].2);
        }
        // println!("{} {}", dirs[index].0, sets.get(&dir).unwrap().len());
        let Reverse(g) = sets.get_mut(&dir).unwrap().pop().unwrap();
        last_x = x + dir.0 * g;
        last_y = y + dir.1 * g;
        // println!("{} {} {} {} {:?}", i + 1, last_x, last_y, g, dirs[index]);
        // println!("{} {}", dirs[index].0, sets.get(&dir).unwrap().len());
        
        index += 1;
        index %= dirs.len();
    }
    (last_x, last_y)
}

fn viewable(map: &Vec<(i32, i32)>, x: i32, y: i32) -> i32 {
    let mut set = HashSet::new();
    for (ax, ay) in map {
        let dy = ay - y;
        let dx = ax - x;
        let g = num_integer::gcd(dy, dx);
        if g == 0 {
            continue;
        }
        set.insert((dx / g, dy / g));
    }
    set.len() as i32
}

fn parse_map(map: Vec<&str>) -> (Vec<(i32, i32)>, i32, i32) {
    let mut result = Vec::new();
    for (y, line) in map.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                result.push((x as i32, y as i32));
            }
        }
    }
    (result, map[0].len() as i32, map.len() as i32)
}

fn search_whole(map: &Vec<(i32, i32)>, bx: i32, by: i32) -> (i32, i32, i32) {
    let mut max = 0;
    let mut saved_x = 0;
    let mut saved_y = 0;
    for y in 0..by {
        for x in 0..bx {
            let value = viewable(&map, x, y);
            if value > max {
                max = value;
                saved_x = x;
                saved_y = y;
            }
        }
    }
    (max, saved_x, saved_y)
}


fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("File reading failed");
    let (arr, bx, by) = parse_map(contents.trim().split_whitespace().collect());
    let (_counts, px, py) = search_whole(&arr, bx, by);
    println!("{} {}", px, py);
    println!("{:?}", viewable_200(&arr, px, py));
}
