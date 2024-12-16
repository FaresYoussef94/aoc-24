use std::{
    cmp,
    fs::{self},
    io::Write,
};

advent_of_code::solution!(14);

#[derive(Debug)]
struct RobotConfig {
    p_x: i32,
    p_y: i32,
    v_x: i32,
    v_y: i32,
}
pub fn part_one(input: &str) -> Option<u32> {
    solve_one(input, 101, 103)
}

pub fn part_two(input: &str) -> Option<u32> {
    let robot_configs = parser(input);
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("./data/examples/day142.txt")
        .ok()
        .unwrap();

    for i in 7687..=7687 {
        let mut curr_max = 0;
        let starting = format!(
            "============================{}============================\n",
            i
        );
        file.write_all(starting.as_bytes())
            .expect("Unable to write");
        let mut positions_map: Vec<Vec<String>> = vec![vec![".".to_string(); 103]; 101];
        for robot in &robot_configs {
            let v_x = (101 as i32 + robot.v_x) as usize;
            let v_y = (103 as i32 + robot.v_y) as usize;

            let x = (robot.p_x as usize + v_x * i) % 101;
            let y = (robot.p_y as usize + v_y * i) % 103;

            positions_map[x][y] = "#".to_string();
        }

        for i in 0..101 {
            let row = positions_map[i].join("") + "\n";
            file.write_all(row.as_bytes()).expect("Unable to write");
        }

        file.write_all("\n\n".as_bytes()).expect("Unable to write");
    }

    None
}

pub fn solve_one(input: &str, x_max: usize, y_max: usize) -> Option<u32> {
    let robot_configs = parser(input);
    let mut positions_map: Vec<Vec<usize>> = vec![vec![0; y_max]; x_max];

    for robot in robot_configs.iter() {
        let v_x = (x_max as i32 + robot.v_x) as usize;
        let v_y = (y_max as i32 + robot.v_y) as usize;

        let x = (robot.p_x as usize + v_x * 100) % x_max;
        let y = (robot.p_y as usize + v_y * 100) % y_max;

        positions_map[x][y] += 1;
    }

    let mut result: u32 = 1;
    let mut q_1: u32 = 0;
    let mut q_2: u32 = 0;
    let mut q_3: u32 = 0;
    let mut q_4: u32 = 0;

    for x in 0..x_max {
        if x == x_max / 2 {
            continue;
        }

        for y in 0..y_max {
            if y == y_max / 2 {
                continue;
            }

            if x >= 0 && x < x_max / 2 {
                if y >= 0 && y <= y_max / 2 {
                    q_1 += positions_map[x][y] as u32;
                } else {
                    q_3 += positions_map[x][y] as u32;
                }
            } else {
                if y >= 0 && y <= y_max / 2 {
                    q_2 += positions_map[x][y] as u32;
                } else {
                    q_4 += positions_map[x][y] as u32;
                }
            }
        }
    }

    result *= cmp::max(1, q_1);
    result *= cmp::max(1, q_2);
    result *= cmp::max(1, q_3);
    result *= cmp::max(1, q_4);

    Some(result)
}

//p=0,4 v=3,-3
fn parser(input: &str) -> Vec<RobotConfig> {
    let robots: Vec<&str> = input.lines().filter((|line| !line.is_empty())).collect();
    let mut robot_configs: Vec<RobotConfig> = Vec::new();

    for robot in robots.iter() {
        let parts: Vec<&str> = robot.split_whitespace().collect();

        let p_part = parts[0].strip_prefix("p=").unwrap_or("");
        let v_part = parts[1].strip_prefix("v=").unwrap_or("");

        let p_values: Vec<i32> = p_part
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();

        let v_values: Vec<i32> = v_part
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();

        robot_configs.push(RobotConfig {
            p_x: p_values[0],
            p_y: p_values[1],
            v_x: v_values[0],
            v_y: v_values[1],
        });
    }

    robot_configs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_one(&advent_of_code::template::read_file("examples", DAY), 11, 7);
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
