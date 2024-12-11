advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let trail_map: Vec<Vec<usize>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split("")
                .filter(|height| !height.is_empty())
                .map(|height| height.to_string().parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    let mut reachable_map: Vec<Vec<isize>> = Vec::new();
    let mut visited_map: Vec<Vec<bool>> = Vec::new();
    for i in 0..trail_map.len() {
        reachable_map.push(Vec::new());
        visited_map.push(Vec::new());
        reachable_map[i] = vec![0; trail_map.len()];
        visited_map[i] = vec![false; trail_map.len()];
    }

    let mut result: u32 = 0;

    for i in 0..trail_map.len() {
        for j in 0..trail_map.len() {
            if trail_map[i][j] == 9 {
                result += navigate(&trail_map, &mut visited_map.clone(), false, i, j);
            }
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let trail_map: Vec<Vec<usize>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split("")
                .filter(|height| !height.is_empty())
                .map(|height| height.to_string().parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    let mut reachable_map: Vec<Vec<isize>> = Vec::new();
    let mut visited_map: Vec<Vec<bool>> = Vec::new();
    for i in 0..trail_map.len() {
        reachable_map.push(Vec::new());
        visited_map.push(Vec::new());
        reachable_map[i] = vec![0; trail_map.len()];
        visited_map[i] = vec![false; trail_map.len()];
    }

    let mut result: u32 = 0;

    for i in 0..trail_map.len() {
        for j in 0..trail_map.len() {
            if trail_map[i][j] == 9 {
                result += navigate(&trail_map, &mut visited_map.clone(), true, i, j);
            }
        }
    }

    Some(result)
}

fn navigate(
    heights_map: &Vec<Vec<usize>>,
    visited: &mut Vec<Vec<bool>>,
    skip_visited: bool,
    i: usize,
    j: usize,
) -> u32 {
    if skip_visited {
        if heights_map[i][j] == 0 {
            visited[i][j] = true;
            return 1;
        }
    } else {
        if heights_map[i][j] == 0 && visited[i][j] {
            visited[i][j] = true;
            return 1;
        }
    }
    if heights_map[i][j] == 0 && (skip_visited && visited[i][j]) {
        visited[i][j] = true;
        return 1;
    }

    if heights_map[i][j] == 0 {
        return 0;
    }

    let next_height = heights_map[i][j] - 1;

    let mut trail_sum: u32 = 0;

    if i > 0 && next_height == heights_map[i - 1][j] {
        trail_sum += navigate(heights_map, visited, skip_visited, i - 1, j);
    }
    if i < heights_map.len() - 1 && next_height == heights_map[i + 1][j] {
        trail_sum += navigate(heights_map, visited, skip_visited, i + 1, j);
    }
    if j > 0 && next_height == heights_map[i][j - 1] {
        trail_sum += navigate(heights_map, visited, skip_visited, i, j - 1);
    }
    if j < heights_map.len() - 1 && next_height == heights_map[i][j + 1] {
        trail_sum += navigate(heights_map, visited, skip_visited, i, j + 1);
    }

    trail_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
