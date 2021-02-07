use std::fs;
use std::collections::HashSet;


#[derive(Clone, Copy, Debug)]
enum Op {
    Flip(),
    Cut(i128),
    Increment(i128),
}

// 119315717514047 is total size
// 101741582076661 is number of reps
// 012345678901234

fn do_n_times(n: i128, ops: &Vec<Op>, size: i128) -> Vec<Op> {
    if n == 0 {
        return Vec::new();
    } else if n == 1 {
        return ops.clone();
    }
    if n % 2 == 1 {
        let mut partial = do_n_times(n - 1, ops, size);
        partial.append(&mut ops.clone());
        for i in 0..10 {
            collapse_ops(&mut partial, size);
        }
        return partial;
    } else {
        let mut partial = do_n_times(n / 2, ops, size);
        partial.append(&mut partial.clone());
        for i in 0..100 {
            collapse_ops(&mut partial, size);
        }
        return partial;
    }
}

fn collapse_ops(ops: &mut Vec<Op>, size: i128) {
    if ops.len() == 1 {
        return;
    }
    let last = ops.pop().unwrap();
    let butlast = ops.pop().unwrap();
    match (butlast, last) {
        (Op::Flip(), Op::Flip()) => {
        },
        (Op::Cut(n1), Op::Cut(n2)) => {
            ops.push(Op::Cut((n1 + n2) % size));
        },
        (Op::Increment(n1), Op::Increment(n2)) => {
            ops.push(Op::Increment((n1 * n2) % size));
        },
        (Op::Cut(n1), Op::Increment(n2)) => {
            ops.push(Op::Increment(n2));
            ops.push(Op::Cut((n1 * n2) % size));
        },
        (Op::Cut(n1), Op::Flip()) => {
            ops.push(Op::Flip());
            ops.push(Op::Cut(size - n1));
        },
        (Op::Flip(), Op::Increment(n2)) => {
            ops.push(Op::Increment(size - n2));
            ops.push(Op::Cut(n2));
        },
        (Op::Increment(n1), Op::Flip()) => {
            ops.push(Op::Increment(size - n1));
            ops.push(Op::Cut(1));
        },
        (_, Op::Cut(_)) => {
            ops.push(butlast.clone());
            collapse_ops(ops, size);
            ops.push(last.clone());
        },
        _ => assert!(false),
    }
}

fn pow_mod(a: i128, pow: i128, m: i128) -> i128 {
    if pow == 0 {
        1
    } else if pow % 2 == 1 {
        (a * pow_mod(a, pow - 1, m)) % m
    } else {
        let root = pow_mod(a, pow / 2, m);
        (root * root) % m
    }
}

fn mod_inv(a: i128, m: i128) -> i128 {
    pow_mod(a, m - 2, m)
}

fn before_new(index: i128, size: i128) -> i128 {
    size - index - 1
}

fn before_cut(n: i128, index: i128, size: i128) -> i128 {
    if n < 0 {
        before_cut(n + size, index, size)
    } else if index >= (size - n) {
        index - (size - n)
    } else {
        index + n
    }
}

fn before_inc(n: i128, index: i128, size: i128) -> i128 {
    // inc takes index i to (i * n) % size
    // given (i * n) % size solve for i
    let inv_n = pow_mod(n, size - 2, size);
    (index * inv_n) % size
}


fn deal_new(stack: Vec<i128>) -> Vec<i128> {
    // brings index i to (size - 1) - i
    let mut new_stack = stack.clone();
    new_stack.reverse();
    new_stack
}

fn deal_cut(n: i128, stack: Vec<i128>) -> Vec<i128> {
    // brings index i to 
    // i >= n ? i - n : i + (size - n)
    //      (i + size - n) % size
    if n < 0 {
        deal_cut(stack.len() as i128 + n, stack)
    } else {
        let (left, right) = stack.split_at(n as usize);
        let mut v = right.to_vec();
        v.append(&mut left.to_vec());
        v
    }
}

fn deal_increment(n: usize, stack: Vec<i128>) -> Vec<i128> {
    // brings index i to (i * n) % size
    let mut result = stack.clone();
    let mut index: usize = 0;
    for i in 0..result.len() {
        result[index] = stack[i];
        index = (index + n) % result.len();
    }
    result
}

fn apply_ops(ops: &Vec<Op>, size: i128) -> Vec<i128> {
    let mut arg = (0..size).collect();
    for op in ops {
        match op {
            Op::Flip() => {
                arg = deal_new(arg);
            },
            Op::Cut(n) => {
                arg = deal_cut(*n, arg);
            },
            Op::Increment(n) => {
                arg = deal_increment(*n as usize, arg);
            }
        }
    }
    arg
}

fn apply_and_print(ops: &Vec<Op>, size: i128) {
    println!("{:?} {:?}", apply_ops(ops, size), ops);
}

fn test_deal_new() {
    let arg = vec![0, 1, 2, 3];
    let exp = vec![3, 2, 1, 0];
    assert!(deal_new(arg) == exp);
    for i in 0..exp.len() {
        assert!(before_new(i as i128, exp.len() as i128) == exp[i]);
    }
}

fn test_deal_cut() {
    let arg = (0..10).collect();
    let exp = vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5];
    assert!(deal_cut(-4, arg) == exp);
    for i in 0..exp.len() {
        assert!(before_cut(-4, i as i128, exp.len() as i128) == 
            exp[i] as i128);
    }
}

fn test_deal_inc() {
    let arg = (0..10).collect();
    let exp = vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3];
    assert!(deal_increment(3, arg) == exp);
    for i in 0..exp.len() {
        assert!(before_inc(-4, i as i128, exp.len() as i128) == 
            exp[i] as i128);
    }
}

fn test_collapse() {
    let mut test = vec![Op::Cut(3), Op::Cut(5)];
    apply_and_print(&test, 13);
    collapse_ops(&mut test, 13);
    apply_and_print(&test, 13);
    
    let mut test = vec![Op::Increment(3), Op::Increment(5)];
    apply_and_print(&test, 13);
    collapse_ops(&mut test, 13);
    apply_and_print(&test, 13);

    let mut test = vec![Op::Cut(5), Op::Flip()];
    apply_and_print(&test, 13);
    collapse_ops(&mut test, 13);
    apply_and_print(&test, 13);
    
    let mut test = vec![Op::Cut(5), Op::Increment(3)];
    apply_and_print(&test, 13);
    collapse_ops(&mut test, 13);
    apply_and_print(&test, 13);

    let mut test = vec![Op::Flip(), Op::Increment(3)];
    apply_and_print(&test, 13);
    collapse_ops(&mut test, 13);
    apply_and_print(&test, 13);

    let mut test = vec![Op::Increment(5), Op::Flip()];
    apply_and_print(&test, 13);
    collapse_ops(&mut test, 13);
    apply_and_print(&test, 13);

}

fn main() {
    test_collapse();

    let contents = fs::read_to_string("input22.txt")
        .expect("File reading failed");
    let mut lines: Vec<&str> = contents
        .trim()
        .split('\n')
        .collect();

    let size: i128 = 119315717514047;
    let times: i128 = 101741582076661;

    let mut operations = Vec::new();

    for line in &lines {
        let tokens: Vec<&str> = line.split(" ").collect();
        if tokens[0] == "cut" {
            let cut_amount: i128 = tokens[1].parse().unwrap();
            operations.push(Op::Cut(cut_amount));
        } else {
            if tokens[1] == "into" {
                operations.push(Op::Flip());
            } else {
                let increment_amount = tokens[3].parse().unwrap();
                operations.push(Op::Increment(increment_amount));
            }
        }
    }

    for i in 0..1000 {
        collapse_ops(&mut operations, size);
    }
    
    println!("{}", operations.len());
    println!("{:?}", operations);

    let mut result = do_n_times(times, &operations, size);

    println!("{}", result.len());
    println!("{:?}", result);

    if let Op::Cut(n2) = result.pop().unwrap() {
        if let Op::Increment(n1) = result.pop().unwrap() {
            println!("{}", before_inc(n1, before_cut(n2, 2020, size), size));
        }
    }

}
