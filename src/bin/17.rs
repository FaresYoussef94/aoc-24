use core::panic;

advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<String> {
    let (mut reg_a, mut reg_b, mut reg_c, program) = parse_input(input);

    let mut i = 0;
    let mut result = String::new();

    while i < program.len() {
        let prog = program[i];
        let operand = get_combo_operand(reg_a, reg_b, reg_c, program[i + 1]);

        match prog {
            0 => reg_a /= 2_u32.pow(operand),
            1 => reg_b ^= program[i + 1],
            2 => reg_b = operand % 8,
            3 => {
                if reg_a != 0 {
                    i = program[i + 1] as usize;
                    continue;
                }
            }
            4 => reg_b ^= reg_c,
            5 => {
                result.push_str(&(operand % 8).to_string());
                result.push_str(",");
            }
            6 => reg_b = reg_a / (2_u32.pow(operand)),
            7 => reg_c = reg_a / (2_u32.pow(operand)),
            _ => panic!("Program {} is not supported", prog),
        }

        i += 2;
    }

    Some(result[..result.len() - 1].to_string())
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_reg_a, _reg_b, _reg_c, program) = parse_input(input);

    match find_register_rec(&program, program.len() - 1, &vec![0]) {
        Some(mut register) => {
            register.sort();
            Some(register[0])
        }
        None => None,
    }
}

fn find_register_rec(program: &Vec<u32>, i: usize, p_a: &Vec<u64>) -> Option<Vec<u64>> {
    if p_a.is_empty() {
        return None;
    }

    let mut matches = vec![];
    for a in p_a.iter() {
        for b in 0..8 {
            let la = (a << 3) | b;
            let sa = simulate_loop(la);
            if sa == program[i] as u64 {
                matches.push(la);
            }
        }
    }

    if i == 0 {
        Some(matches)
    } else {
        find_register_rec(program, i - 1, &matches)
    }
}

fn simulate_loop(a: u64) -> u64 {
    let mut b = a % 8;
    b ^= 2;
    let c = a / (2_u64.pow(b.try_into().unwrap()));
    b ^= c;
    b ^= 3;
    b % 8
}

fn get_combo_operand(reg_a: u32, reg_b: u32, reg_c: u32, literal_val: u32) -> u32 {
    match literal_val {
        0..=3 => literal_val,
        4 => reg_a,
        5 => reg_b,
        6 => reg_c,
        _ => panic!("Combo of literal val {} is not accepted", literal_val),
    }
}

fn parse_input(input: &str) -> (u32, u32, u32, Vec<u32>) {
    let mut reg_a: u32 = 0;
    let mut reg_b: u32 = 0;
    let mut reg_c: u32 = 0;
    let mut program: Vec<u32> = Vec::new();

    for line in input.lines() {
        if line.starts_with("Register A") {
            let parts: Vec<&str> = line.split(": ").collect();
            if parts.len() != 2 {
                panic!("Invalid register line: {}", line);
            }

            let reg_value: u32 = match parts[1].parse() {
                Ok(val) => val,
                Err(_) => panic!("Invalid register value: {}", parts[1]),
            };

            reg_a = reg_value;
        } else if line.starts_with("Register B") {
            let parts: Vec<&str> = line.split(": ").collect();
            if parts.len() != 2 {
                panic!("Invalid register line: {}", line);
            }

            let reg_value: u32 = match parts[1].parse() {
                Ok(val) => val,
                Err(_) => panic!("Invalid register value: {}", parts[1]),
            };

            reg_b = reg_value;
        } else if line.starts_with("Register C") {
            let parts: Vec<&str> = line.split(": ").collect();
            if parts.len() != 2 {
                panic!("Invalid register line: {}", line);
            }

            let reg_value: u32 = match parts[1].parse() {
                Ok(val) => val,
                Err(_) => panic!("Invalid register value: {}", parts[1]),
            };

            reg_c = reg_value;
        } else if line.starts_with("Program:") {
            let program_part: Vec<&str> = line.split(": ").collect();
            program = program_part[1]
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect();
        }
    }

    (reg_a, reg_b, reg_c, program)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35073));
    }
}
