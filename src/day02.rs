use std::fs;
// use std::cmp;

fn run_int_code(position: usize, mut program: Vec<i32>) -> Vec<i32> {
    // println!("{} {:?}", position, program);
    let instr = program[position];
    if instr == 99 {
        return program;
    } else if instr == 1 {
        let update = program[position + 3] as usize;
        let pos1 = program[position + 1] as usize;
        let pos2 = program[position + 2] as usize;
        program[update] = 
            program[pos1] + program[pos2];
    } else if instr == 2 {
        let update = program[position + 3] as usize;
        let pos1 = program[position + 1] as usize;
        let pos2 = program[position + 2] as usize;
        program[update] =
            program[pos1] * program[pos2];
    } else {
        return program;
    }
    run_int_code(position + 4, program)
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("File reading failed");
    let stripped: String = contents.split_whitespace().collect();
    let arr: Vec<i32> = stripped.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    println!("Hello, world! {}", 0);
    for noun in 1..100 {
        for verb in 1..100 {
            let mut program = arr.clone();
            program[1] = noun;
            program[2] = verb;
            let result = run_int_code(0, program);
            if result[0] == 19690720 {
                println!("{}", 100 * noun + verb);
            }
        }
    }
}
