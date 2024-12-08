use std::{
    collections::{HashMap, HashSet},
    result, usize,
};

advent_of_code::solution!(8);

#[derive(Debug)]
struct Loc {
    x: usize,
    y: usize,
}

pub fn part_one(input: &str) -> Option<u32> {
    let matrix: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let mut antinodes_loc: HashSet<String> = HashSet::new();
    let mut antenna_loc: HashMap<char, Vec<Loc>> = HashMap::new();

    for (i, e) in matrix.iter().enumerate() {
        for (j, f) in e.iter().enumerate() {
            if f != &'.' {
                if antenna_loc.contains_key(f) {
                    antenna_loc.get_mut(f).unwrap().push(Loc { x: i, y: j });
                } else {
                    let mut new_vec: Vec<Loc> = Vec::new();
                    new_vec.push(Loc { x: i, y: j });
                    antenna_loc.insert(*f, new_vec);
                }
            }
        }
    }

    for (anetnna, locations) in antenna_loc.iter() {
        for i in 0..locations.len() {
            for j in 0..i {
                let first_loc = locations.get(i).unwrap();
                let second_loc = locations.get(j).unwrap();
                //loc_ one -> first_loc -> second_loc -> last_loc
                let x_diff =
                    i32::try_from(first_loc.x).unwrap() - i32::try_from(second_loc.x).unwrap();
                let y_diff =
                    i32::try_from(first_loc.y).unwrap() - i32::try_from(second_loc.y).unwrap();

                let pre_loc_one_x = x_diff + i32::try_from(first_loc.x).unwrap();
                let pre_loc_one_y = y_diff + i32::try_from(first_loc.y).unwrap();

                let post_loc_one_x = i32::try_from(second_loc.x).unwrap() - x_diff;
                let post_loc_one_y = i32::try_from(second_loc.y).unwrap() - y_diff;

                let pre_loc = get_antinode_loc(&matrix, pre_loc_one_x, pre_loc_one_y);
                let post_loc = get_antinode_loc(&matrix, post_loc_one_x, post_loc_one_y);

                if pre_loc.is_some() {
                    let pre_loc_value = pre_loc.unwrap();
                    antinodes_loc.insert(format!("{}-{}", pre_loc_value.x, pre_loc_value.y));
                }
                if post_loc.is_some() {
                    let post_loc_value = post_loc.unwrap();
                    antinodes_loc.insert(format!("{}-{}", post_loc_value.x, post_loc_value.y));
                }
            }
        }
    }

    Some(antinodes_loc.len().try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let matrix: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let mut antinodes_loc: HashSet<String> = HashSet::new();
    let mut antenna_loc: HashMap<char, Vec<Loc>> = HashMap::new();

    for (i, e) in matrix.iter().enumerate() {
        for (j, f) in e.iter().enumerate() {
            if f != &'.' {
                if antenna_loc.contains_key(f) {
                    antenna_loc.get_mut(f).unwrap().push(Loc { x: i, y: j });
                } else {
                    let mut new_vec: Vec<Loc> = Vec::new();
                    new_vec.push(Loc { x: i, y: j });
                    antenna_loc.insert(*f, new_vec);
                }
            }
        }
    }

    for (anetnna, locations) in antenna_loc.iter() {
        for i in 0..locations.len() {
            for j in 0..i {
                let first_loc = locations.get(i).unwrap();
                let second_loc = locations.get(j).unwrap();
                let x_diff =
                    i32::try_from(first_loc.x).unwrap() - i32::try_from(second_loc.x).unwrap();
                let y_diff =
                    i32::try_from(first_loc.y).unwrap() - i32::try_from(second_loc.y).unwrap();

                let mut curr_x = i32::try_from(first_loc.x).unwrap();
                let mut curr_y = i32::try_from(first_loc.y).unwrap();

                while get_antinode_loc(&matrix, curr_x, curr_y).is_some() {
                    let loc = get_antinode_loc(&matrix, curr_x, curr_y).unwrap();
                    antinodes_loc.insert(format!("{}-{}", loc.x, loc.y));
                    curr_x += x_diff;
                    curr_y += y_diff;
                }

                curr_x = i32::try_from(second_loc.x).unwrap();
                curr_y = i32::try_from(second_loc.y).unwrap();

                while get_antinode_loc(&matrix, curr_x, curr_y).is_some() {
                    let loc = get_antinode_loc(&matrix, curr_x, curr_y).unwrap();
                    antinodes_loc.insert(format!("{}-{}", loc.x, loc.y));
                    curr_x -= x_diff;
                    curr_y -= y_diff;
                }
            }
        }
    }

    Some(antinodes_loc.len().try_into().unwrap())
}

fn get_antinode_loc(matrix: &Vec<Vec<char>>, i: i32, j: i32) -> Option<Loc> {
    let matrix_len = (matrix.len() - 1) as i32;

    if i < 0 || i > matrix_len || j < 0 || j > matrix_len {
        return None;
    }

    let i_usize = usize::try_from(i).unwrap();
    let j_usize = usize::try_from(j).unwrap();

    Some(Loc {
        x: i_usize,
        y: j_usize,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
