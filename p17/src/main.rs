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
                let numerator = registers[0];
                let denominator = 2_u32.pow(combo_operand(operand, &registers));
                registers[0] = numerator / denominator;
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
                let numerator = registers[0];
                let denominator = 2_u32.pow(combo_operand(operand, &registers));
                registers[1] = numerator / denominator;
            },
            7 => { // cdv
                let numerator = registers[0];
                let denominator = 2_u32.pow(combo_operand(operand, &registers));
                registers[2] = numerator / denominator;
            },
            _ => panic!("Invalid opcode {}", opcode)
        }

        instruction_pointer += 2;
    }

    output.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(",")
}

fn combo_operand(n: u8, registers: &Vec<u32>) -> u32 {
    match n {
        0..=3 => n as u32,
        4 => registers[0],
        5 => registers[1],
        6 => registers[2],
        _ => panic!("Invalid combo operand")
    }
}

#[test]
fn test_output() {
    let (mut registers, programs) = parse("1");
    assert_eq!(
        output(&mut registers, &programs),
        String::from("4,6,3,5,6,3,5,2,1,0")
    )
}
