use std::fs;

fn main() {
    let (a, b, c, programs) = parse("input");

    let mut output = format!("{:?}", output(a, b, c, &programs));
    output = output.replace("[", "");
    output = output.replace("]", "");
    output = output.replace(" ", "");

    println!("p1 {:?}", output);
    println!("p2 {:?}", find_a_register(0, b, c, 1, &programs));
}

fn parse(input: &'static str) -> (u64, u64, u64, Vec<u8>) {
    let data = fs::read_to_string(input).unwrap();
    let (registers_r, programs_r) = data.split_once("\n\n").unwrap();

    let registers: Vec<u64> = registers_r
        .split("\n")
        .map(|register| {
            let (_, id) = register.split_once(": ").unwrap();

            id.parse::<u64>().unwrap()
        })
        .collect();

    let (_, ids) = programs_r.split_once(": ").unwrap();
    let programs = ids
        .trim()
        .split(",")
        .map(|id| id.parse::<u8>().unwrap())
        .collect();

    (registers[0], registers[1], registers[2], programs)
}

fn output(ia: u64, ib: u64, ic: u64, programs: &Vec<u8>) -> Vec<u8> {
    let mut a = ia;
    let mut b = ib;
    let mut c = ic;
    let mut instruction_pointer = 0;
    let mut output = vec![];

    while instruction_pointer < programs.len() {
        let opcode = programs[instruction_pointer];
        let operand = programs[instruction_pointer + 1];

        match opcode {
            0 => a = divide(operand, a, b, c),            // adv
            1 => b = b ^ (operand as u64),                // bxl
            2 => b = combo_operand(operand, a, b, c) % 8, // bst,
            3 => { // jnz
                if a != 0 {
                    instruction_pointer = operand as usize;
                    continue
                }
            },
            4 => b = b ^ c, // bxc
            5 => { // out
                let o = combo_operand(operand, a, b, c) % 8;
                output.push(o as u8);
            },
            6 => b = divide(operand, a, b, c), // bdv
            7 => c = divide(operand, a, b, c), // cdv
            _ => panic!("Invalid opcode {}", opcode)
        }

        instruction_pointer += 2;
    }

    output
}

#[test]
fn test_output() {
    let (a, b, c, programs) = parse("1");
    assert_eq!(
        output(a, b, c, &programs),
        vec![4,6,3,5,6,3,5,2,1,0]
    )
}

fn combo_operand(operand: u8, a: u64, b: u64, c: u64) -> u64 {
    match operand {
        0..=3 => operand as u64,
        4 => a,
        5 => b,
        6 => c,
        _ => panic!("Invalid combo operand")
    }
}

fn divide(operand: u8, a: u64, b: u64, c: u64) -> u64 {
    let combo = combo_operand(operand, a, b, c);
    let denominator = 2_u64.pow(combo as u32);

    a / denominator
}

fn find_a_register(a: u64, b: u64, c: u64, n: usize, programs: &Vec<u8>) -> u64 {
    if n > programs.len() {
        return a
    }

    for i in 0..8 {
        let ia = (a << 3) | i;
        let out = output(ia, b, c, programs);

        let mut slice = vec![];
        let m = programs.len();
        for i in (m-n..m).rev() {
            slice.insert(0, programs[i]);
        }

        if out == slice {
            let result = find_a_register(ia, b, c, n + 1, programs);
            if result != 0 {
                return result
            }
        }
    }

    0
}

#[test]
fn test_find_a_register() {
    let (_, b, c, programs) = parse("2");
    assert_eq!(
        find_a_register(0, b, c, 1, &programs),
        117440
    )
}
