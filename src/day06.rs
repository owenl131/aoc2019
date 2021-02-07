use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn bfs(start: &str, target: &str, graph: &HashMap<&str, Vec<&str>>) -> i32 {
    let mut visited = HashSet::new();
    let mut queue: VecDeque<(i32, &str)> = VecDeque::new();
    visited.insert(start);
    queue.push_back((0, start));
    while let Some(next) = queue.pop_front() {
        let (dist, node) = next;
        if node == target {
            return dist;
        }
        if let Some(neighbours) = graph.get(node) {
            for direct in neighbours {
                if !visited.contains(direct) {
                    queue.push_back((dist + 1, direct));
                    visited.insert(direct);
                }
            }
        }
    }
    -1
}

fn count_orbits(node: &str, graph: &HashMap<&str, Vec<&str>>) -> (i32, i32) {
    println!("{}", node);
    let mut result = 0;
    let mut objects = 1;
    match graph.get(node) {
        Some(neighbours) => for direct in neighbours {
            let (orbits, objs) = count_orbits(direct, graph);
            result += orbits + objs;
            objects += objs;
        },
        None => ()
    }
    (result, objects)
}

fn parse_input(contents: &str) -> HashMap<&str, Vec<&str>> {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    for orbits in contents.split_whitespace() {
        let mut iter = orbits.split(")");
        let a: &str = iter.next().unwrap();
        let b: &str = iter.next().unwrap();
        println!("{} {}", a, b);
        match graph.get_mut(a) {
            Some(v) => v.push(b),
            None => { 
                let mut v: Vec<&str> = Vec::new();
                v.push(b);
                graph.insert(a, v);
            }
        }
        match graph.get_mut(b) {
            Some(v) => v.push(a),
            None => { 
                let mut v: Vec<&str> = Vec::new();
                v.push(a);
                graph.insert(b, v);
            }
        }
    }
    graph
}

fn main() {
    let contents = fs::read_to_string("sample.txt")
        .expect("File reading failed");
    let graph = parse_input(contents.trim());
    // println!("{:?}", graph);
    // println!("{:?}", count_orbits("COM", &graph));
    println!("{}", bfs("YOU", "SAN", &graph));
}
