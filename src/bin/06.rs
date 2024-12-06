use std::{collections::HashSet, usize};

advent_of_code::solution!(6);

enum Direction {
    UP,
    Right,
    Down,
    Left,
}

pub fn part_one(input: &str) -> Option<u32> {
    //i vertical, j horizontal
    let map: Vec<Vec<String>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().map(String::from).collect())
        .collect();

    let (i, j) = get_starting_position(&map);
    let mut distinct_locations: HashSet<String> = HashSet::new();
    get_guard_trail(&map, i, j, &Direction::UP, &mut distinct_locations);

    Some(u32::try_into(distinct_locations.len().try_into().unwrap()).unwrap())
}

//S -> starting position
//U -> going up
//R -> going right
//D -> going down
//L -> going left

pub fn part_two(input: &str) -> Option<u32> {
    //i vertical, j horizontal
    let map: Vec<Vec<String>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().map(String::from).collect())
        .collect();

    let (starting_i, starting_j) = get_starting_position(&map);
    let mut result: u32 = 0;

    for i in 0..map.get(0).unwrap().len() {
        for j in 0..map.get(0).unwrap().len() {
            let mut cloned_map = map.clone();
            let mut visited: HashSet<String> = HashSet::new();
            if i == starting_i && j == starting_j {
                continue;
            }
            if cloned_map[i][j] == "#" {
                continue;
            }
            cloned_map[i][j] = String::from("#");
            if check_loop_two(
                &cloned_map,
                starting_i,
                starting_j,
                &Direction::UP,
                &mut visited,
            ) {
                result += 1;
            }
        }
    }

    Some(result)
}

fn get_guard_trail(
    map: &Vec<Vec<String>>,
    i: usize,
    j: usize,
    direction: &Direction,
    visited: &mut HashSet<String>,
) {
    visited.insert(to_set_value(i, j));

    if did_guard_exit(map, i, j, direction) {
        return;
    }

    let (new_i, new_j, new_direction) = get_next_value(map, i, j, direction);
    get_guard_trail(map, new_i, new_j, &new_direction, visited);
}

fn check_loop_two(
    map: &Vec<Vec<String>>,
    i: usize,
    j: usize,
    direction: &Direction,
    visited: &mut HashSet<String>,
) -> bool {
    if did_guard_exit(map, i, j, direction) {
        return false;
    } else if visited.contains(&to_set_value_with_direction(i, j, direction)) {
        return true;
    }

    visited.insert(to_set_value_with_direction(i, j, direction));

    let (new_i, new_j, new_dir) = get_next_value(map, i, j, direction);
    check_loop_two(map, new_i, new_j, &new_dir, visited)
}

fn get_starting_position(map: &Vec<Vec<String>>) -> (usize, usize) {
    for i in 0..map.len() {
        for j in 0..map.len() {
            if *map.get(i).unwrap().get(j).unwrap() == "^" {
                return (i, j);
            }
        }
    }
    (0, 0)
}

fn get_orientation(map: &Vec<Vec<String>>, i: usize, j: usize, direction: &Direction) -> Direction {
    match direction {
        Direction::UP => {
            if i == 0 || "#" != *map.get(i - 1).unwrap().get(j).unwrap() {
                Direction::UP
            } else {
                get_orientation(map, i, j, &Direction::Right)
            }
        }
        Direction::Right => {
            if j == map.len() - 1 || "#" != *map.get(i).unwrap().get(j + 1).unwrap() {
                Direction::Right
            } else {
                get_orientation(map, i, j, &Direction::Down)
            }
        }
        Direction::Down => {
            if i == map.len() - 1 || "#" != *map.get(i + 1).unwrap().get(j).unwrap() {
                Direction::Down
            } else {
                get_orientation(map, i, j, &Direction::Left)
            }
        }
        Direction::Left => {
            if j == 0 || "#" != *map.get(i).unwrap().get(j - 1).unwrap() {
                Direction::Left
            } else {
                get_orientation(map, i, j, &Direction::UP)
            }
        }
    }
}

fn get_next_value(
    map: &Vec<Vec<String>>,
    i: usize,
    j: usize,
    direction: &Direction,
) -> (usize, usize, Direction) {
    let orientation = get_orientation(map, i, j, direction);
    match orientation {
        Direction::UP => (i - 1, j, orientation),
        Direction::Right => (i, j + 1, orientation),
        Direction::Down => (i + 1, j, orientation),
        Direction::Left => (i, j - 1, orientation),
    }
}

fn did_guard_exit(map: &Vec<Vec<String>>, i: usize, j: usize, direction: &Direction) -> bool {
    (matches!(direction, Direction::UP) && i == 0)
        || (matches!(direction, Direction::Right) && j == map.len() - 1)
        || (matches!(direction, Direction::Down) && i == map.len() - 1)
        || (matches!(direction, Direction::Left) && j == 0)
}

fn to_set_value(i: usize, j: usize) -> String {
    format!("{}-{}", i, j)
}
fn to_set_value_with_direction(i: usize, j: usize, direction: &Direction) -> String {
    format!("{}-{}-{}", i, j, to_str_direction(direction))
}

fn to_str_direction(direction: &Direction) -> &str {
    match direction {
        Direction::UP => "U",
        Direction::Right => "R",
        Direction::Down => "D",
        Direction::Left => "L",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
