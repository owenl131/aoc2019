use std::fs;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use itertools::Itertools;

#[derive(Debug)]
struct Machine {
    id: usize,
    ip: usize,
    ram: Vec<i32>,
    inputs: Receiver<i32>,
    outputs: Sender<i32>,
    last_output: i32
}

fn get_index(m: &Machine, index: usize, is_immediate: bool) -> i32 {
    if is_immediate {
        get_immediate(m, index)
    } else {
        get_arg(m, index)
    }
}

fn get_arg(m: &Machine, index: usize) -> i32 {
    return m.ram[m.ram[m.ip + index] as usize];
}

fn get_immediate(m: &Machine, index: usize) -> i32 {
    return m.ram[m.ip + index];
}

fn parse_instr(instr: i32) -> (i32, bool, bool, bool) {
    (instr % 100,
        ((instr / 100) % 10) != 0,
        ((instr / 1000) % 10) != 0,
        ((instr / 10000) % 10) != 0)
}

fn get_instr(m: &Machine) -> i32 {
    m.ram[m.ip]
}

fn do_addition(m: &mut Machine) -> bool {
    let (_instr, m1, m2, _m3) = parse_instr(get_instr(m));
    let arg1 = get_index(m, 1, m1);
    let arg2 = get_index(m, 2, m2);
    let output = get_immediate(m, 3) as usize;
    m.ram[output] = arg1 + arg2;
    if output != m.ip {
        m.ip += 4;
    }
    true
}

fn do_multiplication(m: &mut Machine) -> bool {
    let (_instr, m1, m2, _m3) = parse_instr(get_instr(m));
    let arg1 = get_index(m, 1, m1);
    let arg2 = get_index(m, 2, m2);
    let output = get_immediate(m, 3) as usize;
    m.ram[output] = arg1 * arg2;
    if output != m.ip {
        m.ip += 4;
    }
    true
}

fn do_input(m: &mut Machine) -> bool {
    let pos = get_immediate(m, 1) as usize;
    let value = m.inputs.recv().unwrap();
    // println!("Machine {} read {} from input", m.id, value);
    m.ram[pos] = value;
    if pos != m.ip {
        m.ip += 2;
    }
    true
}

fn do_output(m: &mut Machine) -> bool {
    let (_instr, m1, _m2, _m3) = parse_instr(get_instr(m));
    let value = get_index(m, 1, m1);
    // println!("Machine {} outputting {}", m.id, value);
    m.last_output = value;
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
        m.ip = arg2 as usize;
    } else {
        m.ip += 3;
    }
    true
}

fn do_lt(m: &mut Machine) -> bool {
    let (_instr, m1, m2, _m3) = parse_instr(get_instr(m));
    let arg1 = get_index(m, 1, m1);
    let arg2 = get_index(m, 2, m2);
    let output = get_immediate(m, 3) as usize;
    m.ram[output] = if arg1 < arg2 { 1 } else { 0 };
    if output != m.ip {
        m.ip += 4;
    }
    true
}

fn do_eq(m: &mut Machine) -> bool {
    let (_instr, m1, m2, _m3) = parse_instr(get_instr(m));
    let arg1 = get_index(m, 1, m1);
    let arg2 = get_index(m, 2, m2);
    let output = get_immediate(m, 3) as usize;
    m.ram[output] = if arg1 == arg2 { 1 } else { 0 };
    if output != m.ip {
        m.ip += 4;
    }
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
        _ => false
    }
}

fn run_machine(mut m: Machine) -> thread::JoinHandle<Machine> {
    thread::spawn(move || {
        while run_one_step(&mut m) {
    
        }
        // println!("Machine {} terminated.", m.id);
        m
    })
}

fn run_simulation(settings: Vec<i32>, program: Vec<i32>) -> i32 {
    let (my_input, mut next_input) = mpsc::channel();
    let (mut next_output, mut nn_input) = mpsc::channel();
    my_input.send(settings[0]).unwrap();
    let mut joins: Vec<thread::JoinHandle<Machine>> = Vec::new();
    let mut mac: Machine;
    for i in 0..5 {
        if i != 4 {
            next_output.send(settings[i+1]).unwrap();
        }
        mac = Machine {
            id: i,
            ip: 0,
            ram: program.clone(),
            inputs: next_input,
            outputs: if i == 4 { my_input.clone() } else { next_output.clone() },
            last_output: 0
        };
        joins.push(run_machine(mac));
        next_input = nn_input;
        if i == 4 {
            break
        }
        let (new_output, new_input) = mpsc::channel();
        next_output = new_output;
        nn_input = new_input;
    }
    my_input.send(0).unwrap();
    let mut output: i32 = 0;
    for j in joins {
        let m = j.join().unwrap();
        if m.id == 0 {
            output = m.inputs.recv().unwrap();
        }
        // output = m.last_output;
    }
    output
}

fn test_machine(test_program: &str, test_input: i32, test_output: i32) {
    let (my_input, input) = mpsc::channel();
    let (output, my_output) = mpsc::channel();
    let program = parse_input(test_program);
    let mac = Machine {
        id: 0,
        ip: 0,
        ram: program,
        inputs: input,
        outputs: output,
        last_output: 0
    };
    run_machine(mac);
    my_input.send(test_input).unwrap();
    assert!(my_output.recv().unwrap() == test_output);
}

fn parse_input(input: &str) -> Vec<i32> {
    input.split(",").map(|x| x.parse::<i32>().unwrap()).collect()
}

fn main() {
    assert!(parse_instr(1105) == (5, true, true, false));
    // test_machine("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 0, 0);
    // test_machine("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 1, 1);
    /*test_machine(
        "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",
        1, 999);
    test_machine(
        "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",
        8, 1000);
    test_machine(
        "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",
        1000, 1001);*/
    let contents = fs::read_to_string("input.txt")
        .expect("File reading failed");
    let arr = parse_input(contents.trim());
    // let arr2 = parse_input("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5");
    // println!("Simulation done: {}", run_simulation([3,1,2,4,0], arr));
    // println!("Simulation done: {}", run_simulation([4,3,2,1,0], parse_input("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0")));
    let mut max = 0;
    for perm in (5..10).permutations(5) {
        let result = run_simulation(perm, arr.clone());
        if result > max {
            max = result;
        }
    }
    println!("{}", max);
}
