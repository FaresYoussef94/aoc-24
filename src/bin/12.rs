use std::{collections::HashMap, usize};

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
    let garden: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|row| row.chars().collect())
        .collect();

    let mut result: u32 = 0;
    let mut visited = vec![vec!(false; garden.len()); garden.len()];
    let mut regions: Vec<Vec<(usize, usize)>> = Vec::new();

    for i in 0..garden.len() {
        for j in 0..garden.len() {
            let mut curr_regions: Vec<(usize, usize)> = Vec::new();
            iterate_with_regoin(
                &garden,
                &mut visited,
                &mut curr_regions,
                &garden[i][j],
                i,
                j,
            );

            if !curr_regions.is_empty() {
                regions.push(curr_regions);
            }
        }
    }

    for i in 0..regions.len() {
        let region = regions.get(i).unwrap();
        let mut perimeter: u32 = 0;
        for j in 0..region.len() {
            let (k, l) = *region.get(j).unwrap();
            perimeter += cal_perimeter(&garden, k, l);
        }
        result += perimeter * region.len() as u32;
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let garden: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|row| row.chars().collect())
        .collect();

    let mut result: u32 = 0;
    let mut visited = vec![vec!(false; garden.len()); garden.len()];
    let mut regions: Vec<Vec<(usize, usize)>> = Vec::new();

    for i in 0..garden.len() {
        for j in 0..garden.len() {
            let mut curr_regions: Vec<(usize, usize)> = Vec::new();
            iterate_with_regoin(
                &garden,
                &mut visited,
                &mut curr_regions,
                &garden[i][j],
                i,
                j,
            );

            if !curr_regions.is_empty() {
                regions.push(curr_regions);
            }
        }
    }

    for i in 0..regions.len() {
        let region = regions.get(i).unwrap();
        let (a, b) = region.first().unwrap();
        let name = garden[*a][*b];

        let mut horizontal_plane: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
        let mut vertical_plane: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
        region.iter().for_each(|(h, v)| {
            horizontal_plane.entry(*h).or_default().push((*h, *v));
            vertical_plane.entry(*v).or_default().push((*h, *v));
        });

        let mut sides: u32 = 0;

        for (key, plots) in horizontal_plane.into_iter() {
            let mut sorted_plots = plots.clone();
            sorted_plots.sort_by(|(_, b), (_, d)| b.cmp(d));
            sides += cal_horizontal_sides(&garden, key, sorted_plots);
        }

        for (key, plots) in vertical_plane.into_iter() {
            let mut sorted_plots = plots.clone();
            sorted_plots.sort_by(|(a, _), (c, _)| a.cmp(c));
            sides += cal_vertical_sides(&garden, key, sorted_plots);
        }

        result += region.len() as u32 * sides;
    }

    Some(result)
}

fn iterate_with_regoin(
    garden: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    regions: &mut Vec<(usize, usize)>,
    region: &char,
    i: usize,
    j: usize,
) {
    if visited[i][j] {
        return;
    }

    if *region != garden[i][j] {
        return;
    }

    visited[i][j] = true;
    regions.push((i, j));

    if i > 0 {
        iterate_with_regoin(garden, visited, regions, region, i - 1, j);
    }
    if i < garden.len() - 1 {
        iterate_with_regoin(garden, visited, regions, region, i + 1, j);
    }
    if j > 0 {
        iterate_with_regoin(garden, visited, regions, region, i, j - 1);
    }
    if j < garden.len() - 1 {
        iterate_with_regoin(garden, visited, regions, region, i, j + 1);
    }
}

fn cal_perimeter(garden: &Vec<Vec<char>>, i: usize, j: usize) -> u32 {
    let mut perimeter: u32 = 0;
    if i == 0 {
        perimeter += 1;
        perimeter += if garden[i][j] == garden[i + 1][j] {
            0
        } else {
            1
        };
    } else if i == garden.len() - 1 {
        perimeter += 1;
        perimeter += if garden[i][j] == garden[i - 1][j] {
            0
        } else {
            1
        };
    } else {
        perimeter += if garden[i][j] == garden[i - 1][j] {
            0
        } else {
            1
        };
        perimeter += if garden[i][j] == garden[i + 1][j] {
            0
        } else {
            1
        };
    }

    if j == 0 {
        perimeter += 1;
        perimeter += if garden[i][j] == garden[i][j + 1] {
            0
        } else {
            1
        };
    } else if j == garden.len() - 1 {
        perimeter += 1;
        perimeter += if garden[i][j] == garden[i][j - 1] {
            0
        } else {
            1
        };
    } else {
        perimeter += if garden[i][j] == garden[i][j - 1] {
            0
        } else {
            1
        };
        perimeter += if garden[i][j] == garden[i][j + 1] {
            0
        } else {
            1
        };
    }

    perimeter
}

fn cal_horizontal_sides(garden: &Vec<Vec<char>>, i: usize, plots: Vec<(usize, usize)>) -> u32 {
    let mut sides: u32 = 0;

    let (i2, mut prev_j) = plots.first().unwrap();
    let mut previous_upper_edge = is_edge(garden, i, prev_j, 'u');
    let mut previous_lower_edge = is_edge(garden, i, prev_j, 'd');

    sides += if previous_upper_edge { 1 } else { 0 };
    sides += if previous_lower_edge { 1 } else { 0 };

    for k in 1..plots.len() {
        let (m, curr_j) = plots.get(k).unwrap();
        let curr_upper_edge = is_edge(garden, i, *curr_j, 'u');
        let curr_lower_edge = is_edge(garden, i, *curr_j, 'd');

        if curr_upper_edge && (!previous_upper_edge || *curr_j != prev_j + 1) {
            sides += 1;
        }

        if curr_lower_edge && (!previous_lower_edge || *curr_j != prev_j + 1) {
            sides += 1;
        }

        previous_upper_edge = curr_upper_edge;
        previous_lower_edge = curr_lower_edge;
        prev_j = *curr_j;
    }

    sides
}

fn cal_vertical_sides(garden: &Vec<Vec<char>>, j: usize, plots: Vec<(usize, usize)>) -> u32 {
    let mut sides: u32 = 0;

    let (mut prev_i, _) = plots.first().unwrap();
    let mut previous_left_edge = is_edge(garden, prev_i, j, 'l');
    let mut previous_right_edge = is_edge(garden, prev_i, j, 'r');

    sides += if previous_left_edge { 1 } else { 0 };
    sides += if previous_right_edge { 1 } else { 0 };

    for k in 1..plots.len() {
        let (curr_i, _) = plots.get(k).unwrap();
        let curr_left_edge = is_edge(garden, *curr_i, j, 'l');
        let curr_right_edge = is_edge(garden, *curr_i, j, 'r');

        if curr_left_edge && (!previous_left_edge || *curr_i != prev_i + 1) {
            sides += 1;
        }

        if curr_right_edge && (!previous_right_edge || *curr_i != prev_i + 1) {
            sides += 1;
        }

        previous_left_edge = curr_left_edge;
        previous_right_edge = curr_right_edge;
        prev_i = *curr_i;
    }

    sides
}

fn is_edge(garden: &Vec<Vec<char>>, i: usize, j: usize, dir: char) -> bool {
    match dir {
        'u' => {
            if i == 0 {
                return true;
            } else {
                return garden[i][j] != garden[i - 1][j];
            }
        }
        'd' => {
            if i == garden.len() - 1 {
                true
            } else {
                garden[i][j] != garden[i + 1][j]
            }
        }
        'r' => {
            if j == garden.len() - 1 {
                true
            } else {
                garden[i][j] != garden[i][j + 1]
            }
        }
        'l' => {
            if j == 0 {
                true
            } else {
                garden[i][j] != garden[i][j - 1]
            }
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
