// #![feature(map_first_last)]
use std::fs;
use num;
use num_integer;
use std::fmt;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct Planet {
    x: i32, 
    y: i32,
    z: i32,
    vx: i32,
    vy: i32, 
    vz: i32
}

impl fmt::Display for Planet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pos=<x={:2}, y={:2}, z={:2}>, vel=<x={:2}, y={:2}, z={:2}>",
            self.x, self.y, self.z, self.vx, self.vy, self.vz)
    }
}

fn interact(object: Planet, other: Planet) -> Planet {
    let mut result = object.clone();
    result.vx += num::signum(other.x - object.x);
    result.vy += num::signum(other.y - object.y);
    result.vz += num::signum(other.z - object.z);
    result
}

fn simulate(object: Planet) -> Planet {
    let mut result = object.clone();
    result.x += result.vx;
    result.y += result.vy;
    result.z += result.vz;
    result
}

fn energy(object: Planet) -> i32 {
    (object.x.abs() + object.y.abs() + object.z.abs()) *
    (object.vx.abs() + object.vy.abs() + object.vz.abs())
}


fn run_iteration(state: Vec<Planet>) -> Vec<Planet> {
    let mut vel_updated: Vec<Planet> = Vec::new();
    for (i, p) in state.iter().enumerate() {
        let mut p_copy = p.clone();
        for (j, other) in state.iter().enumerate() {
            if i == j {
                continue;
            }
            p_copy = interact(p_copy, *other);
        }
        vel_updated.push(p_copy);
    }
    vel_updated.iter().map(|x| simulate(*x)).collect()
}


fn till_repeat(mut state: Vec<(i32, i32)>) -> i32 {
    let mut states = HashMap::new();
    for i in 0.. {
        let serialized = (state[0], state[1], state[2], state[3]);
        if states.contains_key(&serialized) {
            let it = states.get(&serialized).unwrap();
            println!("Repeated at {} {} {}", i, it, i - it);
            return i - it;
        }
        states.insert(serialized, i);
        // update velocities
        for i in 0..4 {
            for j in 0..4 {
                if i == j {
                    continue;
                }
                state[i].1 += num::signum(state[j].0 - state[i].0);
            }
        }
        for i in 0..4 {
            state[i].0 += state[i].1;
        }
    }
    0
}


fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("File reading failed");
    // let contents = "<x=-1, y=0, z=2>\n<x=2, y=-10, z=-7>\n<x=4, y=-8, z=8>\n<x=3, y=5, z=-1>";
    // let contents = "<x=-8, y=-10, z=0>\n<x=5, y=5, z=10>\n<x=2, y=-7, z=3>\n<x=9, y=-8, z=-3>";
    let mut moons: Vec<Planet> = contents
        .trim()
        .split("\n")
        .map(|x| x.trim_matches('<').trim_matches('>').trim())
        .map(|x| x.split(", ").map(|x| x.split_at(2).1.parse::<i32>().unwrap()).collect::<Vec<i32>>())
        .map(|x| Planet {x: x[0], y: x[1], z: x[2], vx: 0, vy: 0, vz: 0 })
        .collect();

    let a = till_repeat(vec![(-19, 0), (-9, 0), (-4, 0), (1, 0)]) as i128;
    let b = till_repeat(vec![(-4, 0), (8, 0), (5, 0), (9, 0)]) as i128;
    let c = till_repeat(vec![(2, 0), (-16, 0), (-11, 0), (-13, 0)]) as i128;
    println!("{}", num_integer::lcm(a, num_integer::lcm(b, c)));

    let mut states: HashSet<(Planet, Planet, Planet, Planet)> = HashSet::new();

    for i in 0..1 {
        let serializable = (moons[0], moons[1], moons[2], moons[3]);
        if states.contains(&serializable) {
            println!("{}", i);
            break;
        }
        states.insert(serializable);
        moons = run_iteration(moons);
        // for m in &moons {
        //     println!("{}", m);
        // }
        // println!();
    }

    // println!("{:?}", moons);
    println!("{}", moons.iter().map(|x| energy(*x)).sum::<i32>());
}
