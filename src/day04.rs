// use std::fs;
// use std::collections::HashMap;
// use std::collections::HashSet;

/*
Conditions:
6-digits, in the range given
two adjacent digits are the same
left to right digits increase or stay the same
*/
static LOWER_BOUND: i32 = 356261;
static UPPER_BOUND: i32 = 846303;

fn has_adjacent_sames(candidate: String) -> bool {
    let mut last_char = 0;
    let mut reps = 1;
    let bytes = candidate.as_bytes();
    for i in 0..bytes.len() {
        if bytes[i] == last_char {
            reps += 1;
        } else if reps == 2 {
            return true;
        } 
        else {
            reps = 1;
            last_char = bytes[i];
        }
    }
    reps == 2
}

fn all_nondecreasing(candidate: String) -> bool {
    let bytes = candidate.as_bytes();
    for i in 0..(bytes.len() - 1) {
        if bytes[i+1] < bytes[i] {
            return false;
        }
    }
    true
}

fn is_valid_password(candidate: i32) -> bool {
    let candidate_str = candidate.to_string();
    candidate_str.len() == 6 && 
        candidate >= LOWER_BOUND &&
        candidate <= UPPER_BOUND &&
        has_adjacent_sames(candidate_str.clone()) &&
        all_nondecreasing(candidate_str.clone())
}


fn main() {
    assert!(has_adjacent_sames("111122".to_string()));
    assert!(!has_adjacent_sames("123444".to_string()));

    println!("{}", (LOWER_BOUND..(UPPER_BOUND + 1))
        // .into_iter()
        .filter(|x| is_valid_password(*x))
        .count());

    println!("{}", 0);
}
