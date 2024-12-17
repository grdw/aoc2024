use std::fs;

fn main() {
    let (mut registers, programs) = parse("input");
    println!("p1 {}", output(&mut registers, &programs));
}

fn parse(input: &'static str) -> (Vec<u32>, Vec<u8>) {
    let data = fs::read_to_string(input).unwrap();
    let (registers_r, programs_r) = data.split_once("\n\n").unwrap();

    let registers = registers_r
        .split("\n")
        .map(|register| {
            let (_, id) = register.split_once(": ").unwrap();

            id.parse::<u32>().unwrap()
        })
        .collect();

    let (_, ids) = programs_r.split_once(": ").unwrap();
    let programs = ids
        .trim()
        .split(",")
        .map(|id| id.parse::<u8>().unwrap())
        .collect();

    (registers, programs)
}

fn output(registers: &mut Vec<u32>, programs: &Vec<u8>) -> String {
    let mut instruction_pointer = 0;
    let mut output = vec![];

    while instruction_pointer < programs.len() {
        let opcode = programs[instruction_pointer];
        let operand = programs[instruction_pointer + 1];

        match opcode {
            0 => { // adv
                registers[0] = divide(operand, &registers);
            },
            1 => { // bxl
                registers[1] = registers[1] ^ (operand as u32);
            },
            2 => { // bst
                let c = combo_operand(operand, &registers) % 8;
                registers[1] = c;
            },
            3 => { // jnz
                if registers[0] != 0 {
                    instruction_pointer = operand as usize;
                    continue
                }
            },
            4 => { // bxc
                registers[1] = registers[1] ^ registers[2];
            },
            5 => { // out
                let c = combo_operand(operand, &registers) % 8;
                output.push(c);
            },
            6 => { // bdv
                registers[1] = divide(operand, &registers);
            },
            7 => { // cdv
                registers[2] = divide(operand, &registers);
            },
            _ => panic!("Invalid opcode {}", opcode)
        }

        instruction_pointer += 2;
    }

    output.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(",")
}

fn combo_operand(operand: u8, registers: &Vec<u32>) -> u32 {
    match operand {
        0..=3 => operand as u32,
        4 => registers[0],
        5 => registers[1],
        6 => registers[2],
        _ => panic!("Invalid combo operand")
    }
}

fn divide(operand: u8, registers: &Vec<u32>) -> u32 {
    let numerator = registers[0];
    let combo = combo_operand(operand, &registers);
    let denominator = 2_u32.pow(combo);

    numerator / denominator
}

#[test]
fn test_output() {
    let (mut registers, programs) = parse("1");
    assert_eq!(
        output(&mut registers, &programs),
        String::from("4,6,3,5,6,3,5,2,1,0")
    )
}
