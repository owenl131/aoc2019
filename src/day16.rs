use std::fs;
// use std::collections::HashMap;
// use std::collections::HashSet;
use std::cmp;

/*
01234567890123456789
--------------------
12341234123412341234 | 1 3 1 3 1 3 1 3 1 3 
                     |  2 4 2 4 2 4 2 4 2 4

11223344112233441122 | 1 2 3 4 1 2 3 4 1 2 
                     |  1 2 3 4 1 2 3 4 1 2

11122233344411122233 | 1 1 2 3 3 4 1 1 2 3 
                     |  1 2 2 3 4 4 1 2 2 3

11112222333344441111 | 1 1 2 2 3 3 4 4 1 1 
                     |  1 1 2 2 3 3 4 4 1 1

11111222223333344444 | 1 1 1 2 2 3 3 3 4 4 
                     |  1 1 2 2 2 3 3 4 4 4

0 2 4 6 8 0 2 4 6 8
1 2 3 4 1 2 3 4 1 2
1 1 2 2 3 3 4 4 1 1
1 1 1 2 2 2 3 3 3 4
1 1 1 1 2 2 2 2 3 3
1 1 1 1 1 2 2 2 2 2

*/

fn compute_naive(arr: Vec<i32>) -> Vec<i32> {
    let mut output = vec![0; arr.len()];
    for i in 1..arr.len() {
        for j in 0..arr.len() {
            let pat: usize = (j / i) % 4;
            if pat == 1 {
                output[i] += arr[j];
            } else if pat == 3 {
                output[i] -= arr[j];
            }
        }
        output[i] = output[i].abs() % 10;
    }
    output
}

fn compute_harmonic(arr: Vec<i32>) -> Vec<i32> {
    let mut cum_sum = vec![0; arr.len()];
    let mut last = 0;
    for i in 0..arr.len() {
        last += arr[i];
        cum_sum[i] = last;
    }
    let mut output = vec![0; arr.len()];
    for i in 1..arr.len() {
        let mut j = i;
        loop {
            let lb = j;
            if lb >= arr.len() {
                break;
            }
            let ub = cmp::min(lb + i - 1, arr.len() - 1);
            output[i] += cum_sum[ub] - if lb == 0 { 0 } else { cum_sum[lb - 1] };
            j += 2 * i;
            let lb2 = j;
            if lb2 >= arr.len() {
                break;
            }
            let ub2 = cmp::min(lb2 + i - 1, arr.len() - 1);
            output[i] -= cum_sum[ub2] - if lb2 == 0 { 0 } else { cum_sum[lb2 - 1] };
            j += 2 * i;
        }
        output[i] = output[i].abs() % 10;
    }
    output
}

fn compute_phase(arr: &mut Vec<i32>, cum_sum: &mut Vec<i32>) {
    assert!(arr.len() == cum_sum.len());
    for i in 1..arr.len() {
        assert!(cum_sum[i] == cum_sum[i-1] + arr[i]);
    }

    let len = arr.len();
    // should be O(nlogn)
    for i in 5000000..len {
        arr[i] = 0;
        let mut j = i;
        // j += ((len / 2) / (4 * i)) * 4 * i;
        loop {
            let lb = j;
            let ub = cmp::min(lb + i - 1, len - 1);
            if lb >= len || lb > ub {
                break;
            }
            let sum = cum_sum[ub] - if lb == 0 { 0 } else { cum_sum[lb - 1] };
            // println!("{} {}", lb, ub);
            arr[i] += sum;
            j += 2 * i;
            let lb2 = j;
            let ub2 = cmp::min(lb2 + i - 1, len - 1);
            if lb2 >= len || lb2 > ub2 {
                break;
            }
            let sum2 = cum_sum[ub2] - if lb2 == 0 { 0 } else { cum_sum[lb2 - 1] };
            // println!("{} {}", lb2, ub2);
            arr[i] -= sum2;
            j += 2 * i;
        }
    }
    for i in 0..len {
        arr[i] = arr[i].abs() % 10;
        if i == 0 {
            cum_sum[i] = arr[i];
        } else {
            cum_sum[i] = cum_sum[i-1] + arr[i];
        }
    }
    // println!("{:?}", arr);
}

fn parse_input(input_string: &str) -> Vec<i32> {
    let mut output = Vec::new();
    output.push(0);
    for i in 0..input_string.len() {
        output.push(input_string[i..i+1].parse().unwrap());
    }
    output
}

fn repeat(input: Vec<i32>, times: i32) -> Vec<i32> {
    // do not repeat the first character
    let mut output = Vec::new();
    output.push(0);
    for _i in 0..times {
        for a in 1..input.len() {
            output.push(input[a]);
        }
    }
    output
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("File reading failed");
    // let contents = "03036732577212944063491565474664";
    let offset: usize = contents[0..7].parse().unwrap();
    println!("{}", contents.len());
    println!("{}", offset);

    // assert!(repeat(vec![1, 2, 3], 3) == vec![1, 2, 3, 1, 2, 3, 1, 2, 3]);

    assert!(compute_naive(parse_input("12345678")) == parse_input("48226158"));
    assert!(compute_naive(parse_input("48226158")) == parse_input("34040438"));
    assert!(compute_naive(parse_input("34040438")) == parse_input("03415518"));
    
    assert!(compute_harmonic(parse_input("12345678")) == parse_input("48226158"));
    assert!(compute_harmonic(parse_input("48226158")) == parse_input("34040438"));
    assert!(compute_harmonic(parse_input("34040438")) == parse_input("03415518"));
    
    let mut state = parse_input(contents.trim());
    state = repeat(state, 10000);
    println!("Input size {}", state.len());
    let mut cum: Vec<i32> = vec![0; state.len()];
    let mut last = 0;
    for i in 0..state.len() {
        last += state[i];
        cum[i] = last;
    }

    for i in 0..100 {
        // let mut check = vec![0; state.len()];
        // for i in 0..state.len() {
        //     check[i] = state[i];
        // }
        println!("{}", i);
        compute_phase(&mut state, &mut cum);
        // let check_output = compute_harmonic(check);
        // for j in 5100000..state.len() {
        //     assert!(check_output[j] == state[j]);
        // }
        // state = compute_harmonic(state);

        for i in 0..8 {
            print!("{}", state[offset + i + 1]);
        }
        println!();
    }

    for i in 0..8 {
        print!("{}", state[offset + i + 1]);
    }
    println!();
    for i in 0..100 {
        print!("{}", state[i]);
    }
    
}
