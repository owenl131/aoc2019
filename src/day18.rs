use std::fs;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn bfs(map: &Vec<Vec<char>>, position: (i32, i32)) -> Vec<(char, i32)> {
    let mut ret = Vec::new();
    let directions = [(0, -1), (0, 1), (1, 0), (-1, 0)];
    let mut queue: VecDeque<(i32, i32, i32)> = VecDeque::new();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    queue.push_back((position.0, position.1, 0));
    while queue.len() > 0 {
        let (x, y, d) = queue.pop_front().unwrap();
        visited.insert((x, y));
        let ch = map[y as usize][x as usize];
        if ch != '.' && (x != position.0 || y != position.1) {
            ret.push((map[y as usize][x as usize], d));
            continue;
        }
        for (dx, dy) in directions.iter() {
            let nx = x + dx;
            let ny = y + dy;
            if nx < 0 || nx >= map[0].len() as i32 {
                continue;
            } else if ny < 0 || ny >= map.len() as i32 {
                continue;
            } else if visited.contains(&(nx, ny)) {
                continue;
            } else if map[ny as usize][nx as usize] == '#' {
                continue;
            } else {
                visited.insert((nx, ny));
                queue.push_back((nx, ny, d + 1));
            }
        }
    }
    ret
}

fn dijkstra(graph: HashMap<char, Vec<(char, i32)>>, num_keys: i32) -> i128 {
    let mut visited_states = HashSet::new();
    let mut queue: BinaryHeap<(Reverse<i128>, char, char, char, char, i32)> = BinaryHeap::new();
    queue.push((Reverse(0), '@', '!', '$', '%', 0));
    while let Some((Reverse(d), curr1, curr2, curr3, curr4, unlocked)) = queue.pop() {
        let small_state = (curr1, curr2, curr3, curr4, unlocked);
        if visited_states.contains(&small_state) {
            continue;
        }
        visited_states.insert(small_state);
        println!("{} {} {} {} {} {:b}", d, curr1, curr2, curr3, curr4, unlocked);
        if unlocked == (1 << num_keys) - 1 {
            return d;
        }
        for (nb_ref, dist_ref) in graph.get(&curr1).unwrap() {
            let nb = *nb_ref;
            let dist = *dist_ref;
            let mut new_unlocked = unlocked;
            if nb >= 'a' && nb <= 'z' {
                let index = nb as i32 - 'a' as i32;
                new_unlocked |= 1 << index;
            } else if nb >= 'A' && nb <= 'Z' {
                let index = nb as i32 - 'A' as i32;
                let is_unlocked: bool = (unlocked & (1 << index)) != 0;
                if !is_unlocked {
                    continue;
                }
            }
            let small_state = (nb, new_unlocked);
            let new_state = (Reverse(d + dist as i128), nb, curr2, curr3, curr4, new_unlocked);
            queue.push(new_state);
        }
        for (nb_ref, dist_ref) in graph.get(&curr2).unwrap() {
            let nb = *nb_ref;
            let dist = *dist_ref;
            let mut new_unlocked = unlocked;
            if nb >= 'a' && nb <= 'z' {
                let index = nb as i32 - 'a' as i32;
                new_unlocked |= 1 << index;
            } else if nb >= 'A' && nb <= 'Z' {
                let index = nb as i32 - 'A' as i32;
                let is_unlocked: bool = (unlocked & (1 << index)) != 0;
                if !is_unlocked {
                    continue;
                }
            }
            let small_state = (nb, new_unlocked);
            let new_state = (Reverse(d + dist as i128), curr1, nb, curr3, curr4, new_unlocked);
            queue.push(new_state);
        }
        for (nb_ref, dist_ref) in graph.get(&curr3).unwrap() {
            let nb = *nb_ref;
            let dist = *dist_ref;
            let mut new_unlocked = unlocked;
            if nb >= 'a' && nb <= 'z' {
                let index = nb as i32 - 'a' as i32;
                new_unlocked |= 1 << index;
            } else if nb >= 'A' && nb <= 'Z' {
                let index = nb as i32 - 'A' as i32;
                let is_unlocked: bool = (unlocked & (1 << index)) != 0;
                if !is_unlocked {
                    continue;
                }
            }
            let small_state = (nb, new_unlocked);
            let new_state = (Reverse(d + dist as i128), curr1, curr2, nb, curr4, new_unlocked);
            queue.push(new_state);
        }
        for (nb_ref, dist_ref) in graph.get(&curr4).unwrap() {
            let nb = *nb_ref;
            let dist = *dist_ref;
            let mut new_unlocked = unlocked;
            if nb >= 'a' && nb <= 'z' {
                let index = nb as i32 - 'a' as i32;
                new_unlocked |= 1 << index;
            } else if nb >= 'A' && nb <= 'Z' {
                let index = nb as i32 - 'A' as i32;
                let is_unlocked: bool = (unlocked & (1 << index)) != 0;
                if !is_unlocked {
                    continue;
                }
            }
            let small_state = (nb, new_unlocked);
            let new_state = (Reverse(d + dist as i128), curr1, curr2, curr3, nb, new_unlocked);
            queue.push(new_state);
        }
    }
    -1
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("File reading failed");
    let lines: Vec<Vec<char>> = contents
        .trim()
        .split_whitespace()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();
    let mut graph = HashMap::<char, Vec<(char, i32)>>::new();
    for y in 0..lines.len() {
        for x in 0..lines[0].len() {
            if lines[y][x] != '.' && lines[y][x] != '#' {
                graph.insert(
                    lines[y][x],
                    bfs(&lines, (x as i32, y as i32)));
            }
        }
    }
    println!("{:?}", graph);
    let num_keys = graph.keys()
        .filter(|x| { **x >= 'a' && **x <= 'z' })
        .collect::<Vec<&char>>()
        .len() as i32;
    println!("{}", dijkstra(graph, num_keys));
}
