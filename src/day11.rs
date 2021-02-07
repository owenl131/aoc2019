use std::fs;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::collections::HashMap;
use std::collections::HashSet;
// use itertools::Itertools;

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
    // println!("Machine {} read {} from input storing at {}", m.id, value, pos);
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
            // println!("{}", m.base);
            // println!("{} {:?}", m.ip, m.ram);
        }
        // println!("Machine {} terminated.", m.id);
        m
    })
}

fn run_robot(m: Machine, my_input: Sender<i128>, my_output: Receiver<i128>) -> i32 {
    
    let j = run_machine(m);
    let j2 = thread::spawn(move || {
        let mut whites: HashSet<(i32, i32)> = HashSet::new();
        whites.insert((0, 0));
        // let mut painted: HashSet<(i32, i32)> = HashSet::new();
        let mut direction = 1;
        let mut px = 0;
        let mut py = 0;
        let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        // let mut (max_x, min_x, max_y, min_y): i32;
        loop {
            let paint = if whites.contains(&(px, py)) { 1 } else { 0 };
            if my_input.send(paint).is_err() {
                break;
            } //.unwrap();
            let output = my_output.recv();
            if output.is_err() {
                break;
            }
            let color = output.unwrap();
            let turn = my_output.recv().unwrap();
            // println!("{} {}", color, turn);
            // painted.insert((px, py));
            if paint == 1 && color == 0 {
                whites.remove(&(px, py));
            } else if paint == 0 && color == 1 {
                whites.insert((px, py));
            }
            if turn == 0 {
                direction = (direction + 1) % 4;
            } else {
                direction = (direction + 3) % 4;
            }
            px += directions[direction].0;
            py += directions[direction].1;
        }
        for y in (-60..60).rev() {
            for x in -60..60 {
                if whites.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
        println!("{:?}", whites);
        // painted.len() as i32
        0
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
    println!("Test 1 successful");
    test_machine("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 1, 1);
    println!("Test 2 successful");
    test_machine(
        "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",
        1, 999);
    println!("Test 3 successful");
    test_machine(
        "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",
        8, 1000);
    println!("Test 4 successful");
    test_machine(
        "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",
        1000, 1001);
    println!("Test 5 successful");
    test_machine(
        "104,-1125899906842624,99",
        0, -1125899906842624
    );
    println!("Test 6 successful");
    test_machine("109,1,204,-1,99", 0, 109);
    run_test_quine();
    println!("Test 7 successful");
    run_test_big();
    println!("Test 8 successful");
    let contents = fs::read_to_string("input.txt")
        .expect("File reading failed");
    let arr = parse_input(contents.trim());
    let (m, my_input, my_output) = new_machine(arr);
    println!("{}", run_robot(m, my_input, my_output));
}
