use std::fs;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::time;
use std::thread;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;
use std::sync::atomic::{AtomicI64, Ordering};

#[derive(Debug)]
struct Machine {
    id: i32,
    ip: i128,
    ram: HashMap<i128, i128>,
    inputs: Receiver<i128>,
    outputs: Sender<i128>,
    base: i128,
    idle: bool,
    num_queued: Arc<AtomicI64>,
    num_idle: Arc<AtomicI64>,
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
    let value = match m.inputs.try_recv() {
        Ok(x) => {
            if m.idle {
                m.idle = false;
                m.num_idle.fetch_add(-1, Ordering::SeqCst);
            }
            m.num_queued.fetch_add(-1, Ordering::SeqCst);
            x
        },
        _ => {
            if !m.idle {
                m.idle = true;
                m.num_idle.fetch_add(1, Ordering::SeqCst);
            }
            -1
        }
    };
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
    m.num_queued.fetch_add(1, Ordering::SeqCst);
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

fn new_machine(program: HashMap<i128, i128>, counter: Arc<AtomicI64>, idle: Arc<AtomicI64>) -> (Machine, Sender<i128>, Receiver<i128>) {
    let (my_input, input) = mpsc::channel();
    let (output, my_output) = mpsc::channel();
    let mac = Machine {
        id: 0,
        ip: 0,
        ram: program,
        inputs: input,
        outputs: output,
        base: 0,
        idle: false,
        num_queued: counter,
        num_idle: idle,
    };
    (mac, my_input, my_output)
}

fn parse_input(input: &str) -> HashMap<i128, i128> {
    let v: Vec<i128> = input.split(",").map(|x| x.parse::<i128>().unwrap()).collect();
    vec_to_map(v)
}

fn main() {
    let contents = fs::read_to_string("input23.txt")
        .expect("File reading failed");
    let arr = parse_input(contents.trim());
    let mut incoming_channels = HashMap::new();
    let mut outgoing_channels = HashMap::new();
    let mut threads = HashMap::new();
    let mut counter = Arc::new(AtomicI64::new(50));
    let mut idle_count = Arc::new(AtomicI64::new(0));
    let (nat_sender, nat_receiver) = mpsc::channel();
    
    for i in 0..50 {
        let (mac, my_input, my_output) = new_machine(
            arr.clone(), Arc::clone(&counter), Arc::clone(&idle_count));
        let m = run_machine(mac);
        threads.insert(i, m);
        my_input.send(i).unwrap();
        incoming_channels.insert(i, my_output);
        outgoing_channels.insert(i, my_input);
    }
    for i in 0..50 {
        let incoming: Receiver<i128> = incoming_channels.remove(&i).unwrap();
        let channels = outgoing_channels.clone();
        let nat = nat_sender.clone();
        let counter_i = Arc::clone(&counter);
        thread::spawn(move || {
            loop {
                match incoming.recv() {
                    Ok(dest) => {
                        let x = incoming.recv().unwrap();
                        let y = incoming.recv().unwrap();
                        if dest == 255 {
                            // println!("{} {}", x, y);
                            nat.send((x, y)).unwrap();
                            counter_i.fetch_add(-2, Ordering::SeqCst);
                        } else {
                            let dest_channel = channels.get(&dest).unwrap();
                            dest_channel.send(x).unwrap();
                            dest_channel.send(y).unwrap();
                            counter_i.fetch_add(-1, Ordering::SeqCst);
                        }
                    }
                    _ => {
                        break;
                    }
                }
            }
        });
    }
    let channels = outgoing_channels.clone();
    thread::spawn(move || {
        let send_to = channels.get(&0).unwrap();
        let mut llast_y = -2;
        let mut last_x = -1;
        let mut last_y = -1;
        thread::sleep(time::Duration::from_millis(100));
        loop {
            match nat_receiver.try_recv() {
                Ok((x, y)) => {
                    counter.fetch_add(-1, Ordering::SeqCst);
                    println!("{} {}", x, y);
                    last_x = x;
                    last_y = y;
                }
                _ => {

                }
            }
            let counter_now = counter.load(Ordering::SeqCst);
            let idle_now = idle_count.load(Ordering::SeqCst);
            // println!("{}", counter_now);
            if counter_now == 0 && idle_now == 50 {
                println!("IDLE! Sent {} {}", last_x, last_y);
                send_to.send(last_x).unwrap();
                send_to.send(last_y).unwrap();
                counter.fetch_add(2, Ordering::SeqCst);
                if last_y == llast_y {
                    println!("ANS {}", last_y);
                    break;
                }
                llast_y = last_y;
            } else {
                thread::sleep(time::Duration::from_millis(10));
            }
        }
    });
    for i in 0..50 {
        threads.remove(&i).unwrap().join().unwrap();
    }
}
