use core::panic;
use std::u128;

advent_of_code::solution!(13);

#[derive(Debug)]
struct MachineConfig {
    a_x: u32,
    a_y: u32,
    b_x: u32,
    b_y: u32,
    p_x: u32,
    p_y: u32,
}

pub fn part_one(input: &str) -> Option<u32> {
    let machine_configs: Vec<MachineConfig> =
        input.split("\n\n").map(|config| parser(config)).collect();

    let mut result: u32 = 0;
    for config in machine_configs.iter() {
        let final_x: u64 = config.p_x as u64;
        let final_y: u64 = config.p_y as u64;
        let mut curr_a: i32 = 0;
        let mut curr_b: i32 = 100;
        while curr_a < 100 && curr_b >= 0 {
            let curr_x = curr_a as u64 * config.a_x as u64 + curr_b as u64 * config.b_x as u64;
            let curr_y = curr_a as u64 * config.a_y as u64 + curr_b as u64 * config.b_y as u64;
            if final_x == curr_x && final_y == curr_y {
                break;
            }

            if curr_x > final_x || curr_y > final_y {
                curr_b -= 1;
            } else {
                curr_a += 1;
            }
        }

        if curr_a >= 100 || curr_b < 0 {
            continue;
        }
        result += curr_a as u32 * 3 + curr_b as u32;
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u128> {
    let machine_configs: Vec<MachineConfig> =
        input.split("\n\n").map(|config| parser(config)).collect();

    let mut result: u128 = 0;
    for config in machine_configs.iter() {
        let ax = config.a_x as u128;
        let ay = config.a_y as u128;
        let bx = config.b_x as u128;
        let by = config.b_y as u128;
        let px = config.p_x as u128 + 10000000000000;
        let py = config.p_y as u128 + 10000000000000;

        // solving by simultanious equations
        let a_times = ((px * by).abs_diff(py * bx)) / ((ax * by).abs_diff(ay * bx));
        let b_times = (px.abs_diff(ax * a_times)) / bx;

        if ax * a_times + bx * b_times == px && ay * a_times + by * b_times == py {
            result += a_times * 3 + b_times;
        }
    }
    Some(result)
}

fn parser(input: &str) -> MachineConfig {
    let mut a_x: u32 = 0;
    let mut a_y: u32 = 0;
    let mut b_x: u32 = 0;
    let mut b_y: u32 = 0;
    let mut p_x: u32 = 0;
    let mut p_y: u32 = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        if parts.len() == 2 {
            let key = match parts[0].trim() {
                "Button A" => "A".to_string(),
                "Button B" => "B".to_string(),
                "Prize" => "P".to_string(),
                _ => continue,
            };

            let values: Vec<&str> = parts[1].split(", ").collect();

            let x = values[0][2..].parse::<u32>().unwrap();
            let y = values[1][2..].parse::<u32>().unwrap();
            match key.as_str() {
                "A" => {
                    a_x = x;
                    a_y = y;
                }
                "B" => {
                    b_x = x;
                    b_y = y;
                }
                "P" => {
                    p_x = x;
                    p_y = y;
                }
                _ => {
                    panic!("Couldn't parse input: {} {:?} {:?}", key, values, parts)
                }
            }
        }
    }
    MachineConfig {
        a_x,
        a_y,
        b_x,
        b_y,
        p_x,
        p_y,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert!(result.is_some())
    }
}
