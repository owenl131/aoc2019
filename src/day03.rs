use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

fn collect_points(mut points: HashMap<(i32, i32), i32>, mut location: (i32, i32), instr: Vec<&str>) -> HashMap<(i32, i32), i32> {
    let mut steps = 0;
    for i in instr {
        let (dir, dist_str) = i.split_at(1);
        let (dx, dy) = match dir {
            "U" => (0, 1),
            "D" => (0, -1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => (0, 0)
        };
        let dist: i32 = dist_str.parse().unwrap();
        for d in 1..(dist + 1) {
            let loc = (location.0 + dx * d, location.1 + dy * d);
            points.entry(loc).or_insert(steps + d);
        }
        steps += dist;
        location = (location.0 + dx * dist, location.1 + dy * dist);
    }
    points
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("File reading failed");
    let mut iter = contents.split_whitespace();
    let line1: String = iter.next().unwrap().to_string();
    let line2: String = iter.next().unwrap().to_string();
    let mut points1 = HashMap::new();
    let mut points2 = HashMap::new();
    points1 = collect_points(points1, (0, 0), line1.split(",").collect());
    points2 = collect_points(points2, (0, 0), line2.split(",").collect());
    let keys1: HashSet<&(i32, i32)> = points1.keys().into_iter().collect();
    let keys2: HashSet<&(i32, i32)> = points2.keys().into_iter().collect();
    let result = keys1.intersection(&keys2).into_iter().fold(((0, 0), i32::MAX), |acc, a| {
        let dist = points1.get(a).unwrap() + points2.get(a).unwrap();
        if dist < acc.1 {
            (**a, dist)
        } else {
            acc
        }
    });
    
    println!("{}", result.1);
}
