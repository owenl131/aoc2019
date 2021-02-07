use std::fs;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::time;
use std::thread;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use rand::Rng;

#[derive(Debug)]
struct Machine {
    id: i32,
    ip: i128,
    ram: HashMap<i128, i128>,
    inputs: Receiver<i128>,
    outputs: Sender<i128>,
    base: i128
}

fn read(m: &Machine, index: i128) -> i128 {
    assert!(index >= 0);
    let val = *m.ram.get(&index).unwrap_or(&0);
    // println!("[{}] = {}", index, val);
    val
}

fn store(m: &mut Machine, index: i128, value: i128) {
    assert!(index >= 0);
    // println!("[{}] := {}", index, value);
    m.ram.insert(index, value);
}

fn get_index(m: &Machine, index: i128, flag: i128) -> i128 {
    if flag == 1 {
        // immediate value
        get_immediate(m, index)
    } else if flag == 0 {
        // positional value
        get_arg(m, index, 0)
    } else if flag == 2 {
        // relative value
        // println!("Using relative {}", m.base);
        get_arg(m, index, m.base)
    } else {
        panic!("Invalid flag")
    }
}

fn get_arg(m: &Machine, index: i128, offset: i128) -> i128 {
    read(m, get_immediate(m, index) + offset)
}

fn get_immediate(m: &Machine, index: i128) -> i128 {
    read(m, m.ip + index)
}

fn parse_instr(instr: i128) -> (i128, i128, i128, i128) {
    // println!("instr {}", instr);
    assert!(instr > 0);
    (instr % 100,
        ((instr / 100) % 10),
        ((instr / 1000) % 10),
        ((instr / 10000) % 10))
}

fn get_instr(m: &Machine) -> i128 {
    read(m, m.ip)
}

fn do_addition(m: &mut Machine) -> bool {
    let (_instr, m1, m2, m3) = parse_instr(get_instr(m));
    let arg1 = get_index(m, 1, m1);
    let arg2 = get_index(m, 2, m2);
    let output = get_immediate(m, 3) + if m3 == 2 { m.base } else { 0 };
    store(m, output, arg1 + arg2);
    if output != m.ip {
        m.ip += 4;
    }
    true
}

fn do_multiplication(m: &mut Machine) -> bool {
    let (_instr, m1, m2, m3) = parse_instr(get_instr(m));
    let arg1 = get_index(m, 1, m1);
    let arg2 = get_index(m, 2, m2);
    let output = get_immediate(m, 3) + if m3 == 2 { m.base } else { 0 };
    store(m, output, arg1 * arg2);
    if output != m.ip {
        m.ip += 4;
    }
    true
}

fn do_input(m: &mut Machine) -> bool {
    // println!("{} {} {}", read(m, m.ip), read(m, m.ip + 1), read(m, m.ip + 2));
    let (_instr, m1, _m2, _m3) = parse_instr(get_instr(m));
    let pos = get_immediate(m, 1) + if m1 == 2 { m.base } else { 0 };
    let value = m.inputs.recv().unwrap();
    println!("Machine {} read {} from input storing at {}", m.id, value, pos);
    store(m, pos, value);
    if pos != m.ip {
        m.ip += 2;
    }
    true
}

fn do_output(m: &mut Machine) -> bool {
    let (_instr, m1, _m2, _m3) = parse_instr(get_instr(m));
    let value = get_index(m, 1, m1);
    // println!("Machine {} outputting {}", m.id, value);
    m.outputs.send(value).unwrap();
    m.ip += 2;
    true
}

fn do_jmp(m: &mut Machine, jmp_if: bool) -> bool {
    let (_instr, m1, m2, _m3) = parse_instr(get_instr(m));
    let arg1 = get_index(m, 1, m1);
    let arg2 = get_index(m, 2, m2);
    // if jmp_if true then jmp when arg1 is non-zero
    // if jmp_if false then jmp when arg1 is zero
    if (arg1 != 0) == jmp_if {
        m.ip = arg2;
    } else {
        m.ip += 3;
    }
    true
}

fn do_lt(m: &mut Machine) -> bool {
    let (_instr, m1, m2, m3) = parse_instr(get_instr(m));
    let arg1 = get_index(m, 1, m1);
    let arg2 = get_index(m, 2, m2);
    let output = get_immediate(m, 3) + if m3 == 2 { m.base } else { 0 };
    store(m, output, if arg1 < arg2 { 1 } else { 0 });
    if output != m.ip {
        m.ip += 4;
    }
    true
}

fn do_eq(m: &mut Machine) -> bool {
    let (_instr, m1, m2, m3) = parse_instr(get_instr(m));
    let arg1 = get_index(m, 1, m1);
    let arg2 = get_index(m, 2, m2);
    let output = get_immediate(m, 3) + if m3 == 2 { m.base } else { 0 };
    store(m, output, if arg1 == arg2 { 1 } else { 0 });
    if output != m.ip {
        m.ip += 4;
    }
    true
}

fn do_adjust_base(m: &mut Machine) -> bool {
    // println!("Adjust base {} {} {}", read(m, m.ip), read(m, m.ip + 1), read(m, m.ip + 2));
    let (_instr, m1, _m2, _m3) = parse_instr(get_instr(m));
    let arg1 = get_index(m, 1, m1);
    m.base += arg1;
    m.ip += 2;
    true
}

fn run_one_step(m: &mut Machine) -> bool {
    // println!("Running {}: {} {}", m.id, m.ip, get_instr(m));
    let (instr, _m1, _m2, _m3) = parse_instr(get_instr(m));
    match instr {
        1 => do_addition(m),
        2 => do_multiplication(m),
        3 => do_input(m),
        4 => do_output(m),
        5 => do_jmp(m, true),
        6 => do_jmp(m, false),
        7 => do_lt(m),
        8 => do_eq(m),
        9 => do_adjust_base(m),
        99 => false,
        _ => panic!("Invalid instruction")
    }
}

fn vec_to_map(v: Vec<i128>) -> HashMap<i128, i128> {
    let mut result = HashMap::new();
    for i in 0..v.len() {
        result.insert(i as i128, v[i]);
    }
    result
}

fn run_machine(mut m: Machine) -> thread::JoinHandle<Machine> {
    thread::spawn(move || {
        while run_one_step(&mut m) {
        }
        m
    })
}

fn convert(s: &String) -> String {
    s.replace("RRR", "L")
        .split("")
        .collect::<Vec<&str>>()
        .join(",")
        .strip_prefix(",").unwrap()
        .strip_suffix(",").unwrap().to_string()
        .replace("1,1,1,1,1,1,1,1,1,1,1,1", "12")
        .replace("1,1,1,1,1,1,1,1,1,1,1", "11")
        .replace("1,1,1,1,1,1,1,1,1,1", "10")
        .replace("1,1,1,1,1,1,1,1,1", "9")
        .replace("1,1,1,1,1,1,1,1", "8")
        .replace("1,1,1,1,1,1,1", "7")
        .replace("1,1,1,1,1,1", "6")
        .replace("1,1,1,1,1", "5")
        .replace("1,1,1,1", "4")
        .replace("1,1,1", "3")
        .replace("1,1", "2")
        .trim()
        .to_string()
}

fn is_replaceable_single(s: String, st: String) -> Option<(String, String)> {
    let trun = s.trim().to_string();
    for i in 0..trun.len() {
        if trun.as_bytes()[i] == ' ' as u8 {
            break;
        }
        let unit = trun[0..(i+1)].to_string();
        if convert(&unit).len() > 20 {
            continue;
        }
        println!("\t\tTry {}", convert(&unit));
        let mut step = s.clone();
        let mut store = st.clone();
        let mut found = 0;
        while let Some(loc) = step.find(&unit) {
            let empty: String = vec![' '; unit.len()].iter().collect();
            let labelled: String = vec!['c'; unit.len()].iter().collect();
            step.replace_range(loc..(loc+unit.len()), &empty);
            store.replace_range(loc..(loc+unit.len()), &labelled);
            found += 1;
        }
        if found == 0 || found > 10 {
            continue;
        }
        if step.trim().len() == 0 {
            return Some((unit, store));
        }
    }
    None
}

fn is_replaceable(s: String, st: String) -> Option<(String, String, String)> {
    let trun = s.trim().to_string();
    for i in 0..trun.len() {
        if trun.as_bytes()[i] == ' ' as u8 {
            break;
        }
        let unit = trun[0..(i+1)].to_string();
        if convert(&unit).len() > 20 {
            continue;
        }
        println!("\tTry {}", convert(&unit));
        let mut step: String = s.clone();
        let mut store: String = st.clone();
        let mut found = 0;
        while let Some(loc) = step.find(&unit) {
            let empty: String = vec![' '; unit.len()].iter().collect();
            let labelled: String = vec!['b'; unit.len()].iter().collect();
            step.replace_range(loc..(loc+unit.len()), &empty);
            store.replace_range(loc..(loc+unit.len()), &labelled);
            found += 1;
        }
        if found == 0 || found > 10 {
            continue;
        }
        if let Some((unit2, store2)) = is_replaceable_single(step, store) {
            return Some((unit, unit2, store2));
        }
    }

    None
}

fn run_scaffolder2(mut m: Machine, my_input: Sender<i128>, my_output: Receiver<i128>, unit1: String, unit2: String, unit3: String, total: String) -> i32 {
    store(&mut m, 0, 2);
    let j = run_machine(m);
    let j2 = thread::spawn(move || {
        println!("{} {} {}", unit1.len(), unit2.len(), unit3.len());
        for b in total.as_bytes() {
            my_input.send(*b as i128).unwrap();
        }
        my_input.send('\n' as i128).unwrap();
        for b in unit1.as_bytes() {
            my_input.send(*b as i128).unwrap();
        }
        my_input.send('\n' as i128).unwrap();
        for b in unit2.as_bytes() {
            my_input.send(*b as i128).unwrap();
        }
        my_input.send('\n' as i128).unwrap();
        for b in unit3.as_bytes() {
            my_input.send(*b as i128).unwrap();
        }
        my_input.send('\n' as i128).unwrap();
        my_input.send('n' as i128).unwrap();
        my_input.send('\n' as i128).unwrap();

        loop {
            if let Ok(op) = my_output.recv() {
                if op > 255 {
                    println!("{}", op);
                    break;
                }
                print!("{}", op as u8 as char);
            } else {
                break;
            }
            
        }
        0
    });
    // j.join().unwrap();
    j2.join().unwrap()
}

fn run_scaffolder(m: Machine, my_input: Sender<i128>, my_output: Receiver<i128>) -> (String, String, String, String) {
    // store(&mut m, 0, 2);
    let j = run_machine(m);
    let j2 = thread::spawn(move || {
        let dirs: Vec<(i32, i32)> = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];
        let mut dir = 0;

        let mut map = vec![vec!['.'; 64]; 64];
        let mut y = 1;
        let mut x = 1;

        let mut start_y: i32 = 0;
        let mut start_x: i32 = 0;
        loop {
            let output = my_output.recv();
            if output.is_err() {
                break;
            }
            let code = output.unwrap() as u8 as char;
            // print!("{}", code);
            if code == '\n' {
                y += 1;
                x = 1;
            } else {
                if code == '^' {
                    start_x = x;
                    start_y = y;
                }
                map[y as usize][x as usize] = code;
                x += 1;
            }
        }
        // for line in &map {
        //     for ch in line {
        //         print!("{}", ch);
        //     }
        //     println!();
        // }
        let mut result = 0;
        for i in 1..map.len() {
            for j in 1..map[0].len() {
                if map[i][j] != '#' {
                    continue;
                }
                let mut counter = 0;
                if map[i-1][j] == '#' {
                    counter += 1;
                }
                if map[i+1][j] == '#' {
                    counter += 1;
                }
                if map[i][j-1] == '#' {
                    counter += 1;
                }
                if map[i][j+1] == '#' {
                    counter += 1;
                }
                if counter == 4 {
                    result += i * j;
                }
            }
        }
        println!("{}", result);
        let mut steps = Vec::new();
        for i in 0..1000 {
            // println!("{:?}", steps);
            // println!("{} {}", start_x, start_y);
            let next_x = start_x + dirs[dir as usize].0;
            let next_y = start_y + dirs[dir as usize].1;
            let next_tile = map[next_y as usize][next_x as usize];
            if next_tile != '.' {
                // continue along same direction
                steps.push('1');
                map[start_y as usize][start_x as usize] = 'O';
                start_x = next_x as i32;
                start_y = next_y as i32;
            } else {
                let mut counter = 0;
                let i = start_y as usize;
                let j = start_x as usize;
                if map[i-1][j] != '.' {
                    counter += 1;
                }
                if map[i+1][j] != '.' {
                    counter += 1;
                }
                if map[i][j-1] != '.' {
                    counter += 1;
                }
                if map[i][j+1] != '.' {
                    counter += 1;
                }
                if counter == 1 && map[i][j] != '^' {
                    // reached endpoint
                    break;
                }
                dir = (dir + 1) % 4;
                steps.push('R');
                let next_x = start_x + dirs[dir as usize].0;
                let next_y = start_y + dirs[dir as usize].1;
                let next_tile = map[next_y as usize][next_x as usize];
                if next_tile != '#' {
                    steps.push('R');
                    steps.push('R');
                    dir = (dir + 2) % 4;
                }
            }
        }
        map[start_y as usize][start_x as usize] = '$';

        println!("{}", steps.iter().collect::<String>());
        let step_str = steps.iter().collect::<String>().replace("RRR", "L");

        // for line in &map {
        //     for ch in line {
        //         print!("{}", ch);
        //     }
        //     println!();
        // }

        for i in 1..step_str.len() {
            let mut step = step_str.clone();
            let mut st = step_str.clone();
            let unit: String = step_str[0..i].to_string();
            if convert(&unit).len() > 20 {
                continue;
            }
            println!("Try {}", unit);
            let empty: String = vec![' '; unit.len()].iter().collect();
            let labelled: String = vec!['a'; unit.len()].iter().collect();
            let mut found = 0;
            while let Some(loc) = step.find(&unit) {
                step.replace_range(loc..(loc+unit.len()), &empty);
                st.replace_range(loc..(loc+unit.len()), &labelled);
                found += 1;
            }
            if found == 0 || found > 10 {
                continue;
            }
            if let Some((unit2, unit3, store)) = is_replaceable(step, st) {
                println!("Solution {} {} {} {}", 
                    convert(&unit), convert(&unit2), convert(&unit3), store);
                let a_label: String = vec!['a'; unit.len()].iter().collect();
                let b_label: String = vec!['b'; unit2.len()].iter().collect();
                let c_label: String = vec!['c'; unit3.len()].iter().collect();
                let store_final = store
                    .replace(&a_label, "A")
                    .replace(&b_label, "B")
                    .replace(&c_label, "C")
                    .split("")
                    .collect::<Vec<&str>>()
                    .join(",")
                    .strip_prefix(",").unwrap()
                    .strip_suffix(",").unwrap().to_string();
                println!("{}", store_final);
                if store_final.len() > 20 {
                    continue;
                }
                return (convert(&unit), convert(&unit2), convert(&unit3), store_final);
            }
        }
        ("".to_string(), "".to_string(), "".to_string(), "".to_string())
    });
    j.join().unwrap();
    j2.join().unwrap()
}

fn new_machine(program: HashMap<i128, i128>) -> (Machine, Sender<i128>, Receiver<i128>) {
    let (my_input, input) = mpsc::channel();
    let (output, my_output) = mpsc::channel();
    let mac = Machine {
        id: 0,
        ip: 0,
        ram: program,
        inputs: input,
        outputs: output,
        base: 0
    };
    (mac, my_input, my_output)
}

fn test_machine(test_program: &str, test_input: i128, test_output: i128) {
    let program = parse_input(test_program);
    let (mac, my_input, my_output) = new_machine(program);
    let m = run_machine(mac);
    my_input.send(test_input).unwrap();
    assert!(my_output.recv().unwrap() == test_output);
    m.join().unwrap();
}

fn parse_input(input: &str) -> HashMap<i128, i128> {
    let v: Vec<i128> = input.split(",").map(|x| x.parse::<i128>().unwrap()).collect();
    vec_to_map(v)
}

fn run_test_quine() {
    let program = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
    let (mac, _my_input, my_output) = new_machine(parse_input(program.clone()));
    let m = run_machine(mac);
    let mut results: Vec<String> = Vec::new();
    for _i in 0..16 {
        let output: String = my_output.recv().unwrap().to_string();
        results.push(output);
    }
    let cmp = results.join(",");
    assert!(cmp == program);
    m.join().unwrap();
}

fn run_test_big() {
    let program = "1102,34915192,34915192,7,4,7,99,0";
    let (mac, _my_input, my_output) = new_machine(parse_input(program.clone()));
    let m = run_machine(mac);
    let output = my_output.recv().unwrap().to_string();
    assert!(output.len() == 16);
    m.join().unwrap();
}

fn main() {
    assert!(parse_instr(1105) == (5, 1, 1, 0));
    assert!(parse_instr(1008) == (8, 0, 1, 0));
    assert!(parse_instr(209) == (9, 2, 0, 0));
    test_machine("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 0, 0);
    test_machine("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 1, 1);
    test_machine(
        "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",
        1, 999);
    test_machine(
        "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",
        8, 1000);
    test_machine(
        "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",
        1000, 1001);
    test_machine(
        "104,-1125899906842624,99",
        0, -1125899906842624
    );
    test_machine("109,1,204,-1,99", 0, 109);
    run_test_quine();
    run_test_big();
    let contents = fs::read_to_string("input.txt")
        .expect("File reading failed");
    let arr = parse_input(contents.trim());
    let (m, my_input, my_output) = new_machine(arr);
    let (unit1, unit2, unit3, total) = run_scaffolder(m, my_input, my_output);
    let (m2, ipt2, opt2) = new_machine(parse_input(contents.trim()));
    println!("{}", run_scaffolder2(m2, ipt2, opt2, unit1, unit2, unit3, total));
}
