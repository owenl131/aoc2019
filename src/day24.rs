use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

/**********************************************************
  PART 1
**********************************************************/

fn serialize(state: &Vec<Vec<char>>) -> String {
    state
        .into_iter()
        .map(|row| row.into_iter().collect::<String>())
        .collect::<String>()
}

fn evolve(state: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    assert!(state.len() == 5);
    assert!(state[0].len() == 5);
    let mut neighbours = vec![vec![0; 5]; 5];
    let mut result = state.clone();
    // a bug dies unless there is exactly one bug adjacent to it
    // an empty space becomes infested if exactly 
    // one or two bugs are adjacent to it
    for i in 0..5 {
        for j in 0..5 {
            if state[i][j] == '#' {
                if i != 0 {
                    neighbours[i-1][j] += 1;
                }
                if j != 0 {
                    neighbours[i][j-1] += 1;
                }
                if i != 4 {
                    neighbours[i+1][j] += 1;
                }
                if j != 4 {
                    neighbours[i][j+1] += 1;
                }
            }
        }
    }
    for i in 0..5 {
        for j in 0..5 {
            if state[i][j] == '#' && neighbours[i][j] != 1 {
                result[i][j] = '.';
            } else if state[i][j] == '.' && 
                    [1, 2].contains(&neighbours[i][j]) {
                result[i][j] = '#';
            }
        }
    }
    result
}

fn rating(state: &Vec<Vec<char>>) -> i128 {
    let mut result = 0;
    for i in 0..5 {
        for j in 0..5 {
            if state[i][j] == '#' {
                let exp = i * 5 + j;
                result += 1 << exp;
            }
        }
    }
    result
}


/**********************************************************
  PART 2
**********************************************************/

fn evolve2(state: &HashSet<(i32, i32, i32)>) 
        -> HashSet<(i32, i32, i32)> {

    let mut result = HashSet::new();
    let mut neighbours = HashMap::new();

    for &(i, j, level) in state {
        // count neighbours
        if i == 0 {
            // contribute to upper level
            neighbours.entry((1, 2, level + 1))
                .and_modify(|e| { *e += 1})
                .or_insert(1);
        } else {
            // contribute to i-1
            neighbours.entry((i-1, j, level))
                .and_modify(|e| { *e += 1})
                .or_insert(1);
        }
        if j == 0 {
            // contribute to upper level
            neighbours.entry((2, 1, level + 1))
                .and_modify(|e| { *e += 1})
                .or_insert(1);
        } else {
            // contribute to j-1
            neighbours.entry((i, j-1, level))
                .and_modify(|e| { *e += 1})
                .or_insert(1);
        }
        if i == 4 {
            // contribute to upper level
            neighbours.entry((3, 2, level + 1))
                .and_modify(|e| { *e += 1})
                .or_insert(1);
        } else {
            // contribute to i+1
            neighbours.entry((i+1, j, level))
                .and_modify(|e| { *e += 1})
                .or_insert(1);
        }
        if j == 4 {
            // contribute to upper level
            neighbours.entry((2, 3, level + 1))
                .and_modify(|e| { *e += 1})
                .or_insert(1);
        } else {
            // contribute to j+1
            neighbours.entry((i, j+1, level))
                .and_modify(|e| { *e += 1})
                .or_insert(1);
        }
        if i == 1 && j == 2 { // North
            // contribute to lower level
            for k in 0..5 {
                neighbours.entry((0, k, level - 1))
                    .and_modify(|e| { *e += 1})
                    .or_insert(1);
            }
        }
        if i == 3 && j == 2 { // South
            // contribute to lower level
            for k in 0..5 {
                neighbours.entry((4, k, level - 1))
                    .and_modify(|e| { *e += 1})
                    .or_insert(1);
            }
        }
        if i == 2 && j == 1 { // West
            // contribute to lower level
            for k in 0..5 {
                neighbours.entry((k, 0, level - 1))
                    .and_modify(|e| { *e += 1})
                    .or_insert(1);
            }
        }
        if i == 2 && j == 3 { // East
            // contribute to lower level
            for k in 0..5 {
                neighbours.entry((k, 4, level - 1))
                    .and_modify(|e| { *e += 1})
                    .or_insert(1);
            }
        }
    }

    for ((i, j, level), n) in neighbours {
        if i == 2 && j == 2 {
            continue;
        }
        if state.contains(&(i, j, level)) {
            if n == 1 {
                result.insert((i, j, level));
            }
        } else {
            if [1, 2].contains(&n) {
                result.insert((i, j, level));
            }
        }
    }

    result
}

fn main() {
    let contents = fs::read_to_string("input24.txt")
        .expect("File reading failed");
    let mut state: Vec<Vec<char>> = contents
        .trim()
        .split('\n')
        .map(|row| row.chars().collect())
        .collect();
    
        let mut bugs = HashSet::new();
    for i in 0..5 {
        for j in 0..5 {
            if state[i][j] == '#' {
                bugs.insert((i as i32, j as i32, 0));
            }
        }
    }

    let mut visited = HashSet::new();
    visited.insert(serialize(&state));
    println!("{}", serialize(&state));

    for _ in 0..200 {
        bugs = evolve2(&bugs);
    }

    loop {
        state = evolve(&state);
        let new_state_str = serialize(&state);
        // println!("{}", new_state_str);
        if visited.contains(&new_state_str) {
            break;
        }
        visited.insert(new_state_str);
    }

    println!("{}", rating(&state));
    println!("{}", bugs.len());
}
