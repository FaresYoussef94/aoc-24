advent_of_code::solution!(15);
pub fn part_one(input: &str) -> Option<u32> {
    let input_parts: Vec<&str> = input.split("\n\n").collect();
    let mut warehouse_map: Vec<Vec<char>> = input_parts[0]
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let movements: Vec<char> = input_parts[1]
        .lines()
        .collect::<Vec<&str>>()
        .join("")
        .chars()
        .collect();

    for movement in movements.iter() {
        let (i, j) = get_starting_position(&warehouse_map);
        handle_movement(&mut warehouse_map, i, j, *movement);
    }

    let mut result: u32 = 0;

    for i in 0..warehouse_map.len() {
        for j in 0..warehouse_map[0].len() {
            if warehouse_map[i][j] == 'O' {
                result += i as u32 * 100 + j as u32;
            }
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input_parts: Vec<&str> = input.split("\n\n").collect();
    let initial_warehouse_map: Vec<Vec<char>> = input_parts[0]
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let movements: Vec<char> = input_parts[1]
        .lines()
        .collect::<Vec<&str>>()
        .join("")
        .chars()
        .collect();

    let mut warehouse_map: Vec<Vec<char>> = Vec::new();

    for i in 0..initial_warehouse_map.len() {
        let mut row: Vec<char> = Vec::new();
        for j in 0..initial_warehouse_map.len() {
            match initial_warehouse_map[i][j] {
                '#' => {
                    row.push('#');
                    row.push('#');
                }
                'O' => {
                    row.push('[');
                    row.push(']');
                }
                '.' => {
                    row.push('.');
                    row.push('.');
                }
                '@' => {
                    row.push('@');
                    row.push('.');
                }
                _ => {
                    println!("Char not supported: {:?}", initial_warehouse_map[i][j])
                }
            };
        }
        warehouse_map.push(row);
    }

    for movement in movements.iter() {
        let (i, j) = get_starting_position(&warehouse_map);
        if ['^', 'v'].contains(movement) && check_walls(&warehouse_map, i, j, *movement) {
            handle_vertical_movement(&mut warehouse_map, i, j, '.', *movement);
        }
        if ['>', '<'].contains(movement) {
            handle_movement(&mut warehouse_map, i, j, *movement);
        }
    }

    let mut result: u32 = 0;

    for i in 0..warehouse_map.len() {
        for j in 0..warehouse_map[0].len() {
            if warehouse_map[i][j] == '[' {
                result += i as u32 * 100 + j as u32;
            }
        }
    }

    Some(result)
}

fn get_starting_position(warehouse_map: &Vec<Vec<char>>) -> (usize, usize) {
    for i in 0..warehouse_map.len() {
        for j in 0..warehouse_map[0].len() {
            if warehouse_map[i][j] == '@' {
                return (i, j);
            }
        }
    }
    (0, 0)
}

fn handle_movement(warehouse_map: &mut Vec<Vec<char>>, i: usize, j: usize, direction: char) {
    let count = match direction {
        '^' => {
            let mut count = 0;
            while !['.', '#'].contains(&warehouse_map[i - count][j]) {
                count += 1;
            }
            if warehouse_map[i - count][j] == '.' {
                count
            } else {
                0
            }
        }
        '>' => {
            let mut count = 0;
            while !['.', '#'].contains(&warehouse_map[i][j + count]) {
                count += 1;
            }
            if warehouse_map[i][j + count] == '.' {
                count
            } else {
                0
            }
        }
        'v' => {
            let mut count = 0;
            while !['.', '#'].contains(&warehouse_map[i + count][j]) {
                count += 1;
            }
            if warehouse_map[i + count][j] == '.' {
                count
            } else {
                0
            }
        }
        '<' => {
            let mut count = 0;
            while !['.', '#'].contains(&warehouse_map[i][j - count]) {
                count += 1;
            }
            if warehouse_map[i][j - count] == '.' {
                count
            } else {
                0
            }
        }
        _ => 0,
    };

    if count > 0 {
        match direction {
            '^' => handle_up_movement(warehouse_map, i, j, count),
            '>' => handle_right_movement(warehouse_map, i, j, count),
            'v' => handle_down_movement(warehouse_map, i, j, count),
            '<' => handle_left_movement(warehouse_map, i, j, count),
            _ => println!("unrecognized direction"),
        }
    }
}

fn handle_up_movement(warehouse_map: &mut Vec<Vec<char>>, i: usize, j: usize, count: usize) {
    let mut k = 0;
    while k < count {
        let diff = count - k;
        warehouse_map[i - diff][j] = warehouse_map[i + 1 - diff][j];
        k += 1;
    }
    warehouse_map[i][j] = '.';
}

fn handle_down_movement(warehouse_map: &mut Vec<Vec<char>>, i: usize, j: usize, count: usize) {
    let mut k = 0;
    while k < count {
        let diff = count - k;
        warehouse_map[i + diff][j] = warehouse_map[i + diff - 1][j];
        k += 1;
    }
    warehouse_map[i][j] = '.';
}

fn handle_right_movement(warehouse_map: &mut Vec<Vec<char>>, i: usize, j: usize, count: usize) {
    let mut k = 0;
    while k < count {
        let diff = count - k;
        warehouse_map[i][j + diff] = warehouse_map[i][j + diff - 1];
        k += 1;
    }
    warehouse_map[i][j] = '.';
}

fn handle_left_movement(warehouse_map: &mut Vec<Vec<char>>, i: usize, j: usize, count: usize) {
    let mut k = 0;
    while k < count {
        let diff = count - k;
        warehouse_map[i][j - diff] = warehouse_map[i][j - diff + 1];
        k += 1;
    }
    warehouse_map[i][j] = '.';
}

fn check_walls(warehouse_map: &Vec<Vec<char>>, i: usize, j: usize, direction: char) -> bool {
    if warehouse_map[i][j] == '.' {
        return true;
    }

    if warehouse_map[i][j] == '#' {
        return false;
    }

    let next_i = if direction == '^' { i - 1 } else { i + 1 };

    if warehouse_map[i][j] == '[' {
        check_walls(warehouse_map, next_i, j, direction)
            && check_walls(warehouse_map, next_i, j + 1, direction)
    } else if warehouse_map[i][j] == ']' {
        check_walls(warehouse_map, next_i, j - 1, direction)
            && check_walls(warehouse_map, next_i, j, direction)
    } else {
        check_walls(warehouse_map, next_i, j, direction)
    }
}

fn handle_vertical_movement(
    warehouse_map: &mut Vec<Vec<char>>,
    i: usize,
    j: usize,
    new_char: char,
    direction: char,
) {
    if ['.'].contains(&warehouse_map[i][j]) {
        warehouse_map[i][j] = new_char;
        return;
    }
    let old_char = warehouse_map[i][j];
    warehouse_map[i][j] = new_char;

    let next_i = if direction == '^' { i - 1 } else { i + 1 };
    handle_vertical_movement(warehouse_map, next_i, j, old_char, direction);

    if old_char == '[' {
        handle_vertical_movement(warehouse_map, next_i, j + 1, ']', direction);
        warehouse_map[i][j + 1] = '.';
    } else if old_char == ']' {
        handle_vertical_movement(warehouse_map, next_i, j - 1, '[', direction);
        warehouse_map[i][j - 1] = '.';
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
