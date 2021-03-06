use std::fs;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::time;
use std::thread;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;
use std::sync::atomic::{AtomicI64, Ordering};
use std::io::{self, Read, stdin, stdout};

#[derive(Debug)]
struct Machine {
    id: i32,
    ip: i128,
    ram: HashMap<i128, i128>,
    inputs: Receiver<i128>,
    outputs: Sender<i128>,
    base: i128,
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
        }
        m
    })
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
        base: 0,
    };
    (mac, my_input, my_output)
}

fn parse_input(input: &str) -> HashMap<i128, i128> {
    let v: Vec<i128> = input.split(",").map(|x| x.parse::<i128>().unwrap()).collect();
    vec_to_map(v)
}

fn main() -> io::Result<()> {
    let contents = fs::read_to_string("input25.txt")
        .expect("File reading failed");
    let arr = parse_input(contents.trim());
    let (mac, my_input, my_output) = new_machine(arr);
    let thread = run_machine(mac);
    thread::spawn(move || {
        while let Ok(x) = my_output.recv() {
            print!("{}", x as u8 as char);
        }
    });
    let mut buffer = String::new();
    loop {
        // stdout().flush();
        stdin().read_line(&mut buffer).unwrap();
        if buffer == "q" {
            break;
        }
        for c in buffer.chars() {
            my_input.send(c as i128).unwrap();
        }
    }

    thread.join().unwrap();
    Ok(())
}
