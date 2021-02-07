use std::fs;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn get_portal(map: &Vec<Vec<char>>, position: (i32, i32)) -> Option<(char, char, i32)> {
    let (x, y) = position;
    if x < 0 || x >= map[0].len() as i32 || y < 0 || y >= map.len() as i32 {
        None
    } else if map[y as usize][x as usize] != '.' {
        None
    } else {
        // check top bottom left right
        let directions = [(0, -1), (0, 1), (1, 0), (-1, 0)];
        for (dx, dy) in directions.iter() {
            let ch = map[(y + dy) as usize][(x + dx) as usize];
            if ch >= 'A' && ch <= 'Z' {
                let ch2 = map[(y + 2 * dy) as usize][(x + 2 * dx) as usize];
                assert!(ch2 >= 'A' && ch2 <= 'Z');
                let direction = if x < 4 || x > (map[0].len() - 4) as i32 || y < 4 || y > (map.len() - 4) as i32 {
                    -1
                } else {
                    1
                };
                if ch < ch2 {
                    return Some((ch, ch2, direction));
                } else {
                    return Some((ch2, ch, direction));
                }
            }
        }
        None
    }
}

fn bfs(map: &Vec<Vec<char>>, position: (i32, i32)) -> Vec<((char, char, i32), i32)> {
    let mut ret = Vec::new();
    let directions = [(0, -1), (0, 1), (1, 0), (-1, 0)];
    let mut queue: VecDeque<(i32, i32, i32)> = VecDeque::new();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    queue.push_back((position.0, position.1, 0));
    while queue.len() > 0 {
        let (x, y, d) = queue.pop_front().unwrap();
        if visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));
        if x != position.0 || y != position.1 {
            if let Some(label) = get_portal(&map, (x, y)) {
                ret.push((label, d));
                continue;
            }
        }
        for (dx, dy) in directions.iter() {
            let nx = x + dx;
            let ny = y + dy;
            let other = map[ny as usize][nx as usize];
            if nx < 0 || nx >= map[0].len() as i32 {
                continue;
            } else if ny < 0 || ny >= map.len() as i32 {
                continue;
            } else if visited.contains(&(nx, ny)) {
                continue;
            } else if other != '.' {
                continue;
            } else {
                queue.push_back((nx, ny, d + 1));
            }
        }
    }
    ret
}

fn dijkstra(graph: HashMap<(char, char, i32), Vec<((char, char, i32), i32)>>) -> i128 {
    let mut visited_states = HashSet::new();
    let mut queue: BinaryHeap<(Reverse<i128>, (char, char, i32), i32)> = BinaryHeap::new();
    queue.push((Reverse(0), ('A', 'A', -1), 0));
    while let Some((Reverse(d), curr, depth)) = queue.pop() {
        if depth < 0 {
            continue;
        }
        let saved_state = (curr, depth);
        if visited_states.contains(&saved_state) {
            continue;
        }
        visited_states.insert(saved_state);
        if curr == ('Z', 'Z', -1) && depth == 0 {
            // continue;
            return d;
        }
        let (x, y, dir) = curr;
        queue.push((Reverse(d + 1), (x, y, -dir), depth + dir));
        for (nb_ref, dist_ref) in graph.get(&curr).unwrap() {
            let nb = *nb_ref;
            let dist = *dist_ref;
            // if currently not at depth 0, don't use AA and ZZ
            if depth != 0 && (nb == ('A', 'A', -1) || nb == ('Z', 'Z', -1)) {
                continue;
            }
            let new_state = (Reverse(d + dist as i128), nb, depth);
            queue.push(new_state);
        }
    }
    -1
}

fn main() {
    let contents = fs::read_to_string("input20.txt")
        .expect("File reading failed");
    let lines: Vec<Vec<char>> = contents
        .strip_suffix("\n").unwrap()
        .split('\n')
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();
    // println!("{:?}", lines);
    for line in &lines {
        assert!(line.len() == lines[0].len());
    }
    let mut count = 0;
    let mut graph = HashMap::new();
    for y in 0..lines.len() {
        for x in 0..lines[0].len() {
            if let Some(label) = get_portal(&lines, (x as i32, y as i32)) {
                // println!("{:?}", (x, y));
                count += 1;
                assert!(!graph.contains_key(&label));
                graph.entry(label).or_insert(vec![]);
                graph.entry(label).and_modify(|v| v.append(&mut bfs(&lines, (x as i32, y as i32))));
            }
        }
    }
    // println!("{:?}", graph);
    // println!("{}", graph.keys().len());
    println!("{}", dijkstra(graph));
    // println!("{}", count);
}
