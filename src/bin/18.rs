advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<usize> {
    solve_shortest_path(input, 71)
}

fn solve_shortest_path(input: &str, max: usize) -> Option<usize> {
    let mut maze = vec![vec!['.'; max]; max];
    let lines: Vec<&str> = input.lines().collect();
    for i in 0..12 {
        let coordinates: Vec<usize> = lines[i]
            .split(",")
            .map(|point| point.parse::<usize>().unwrap())
            .collect();
        maze[coordinates[0]][coordinates[1]] = '#';
    }

    let mut visited = vec![vec![false; max]; max];
    // let mut distance = vec![vec![usize::MAX; max]; max];
    let mut distance = vec![vec![10000; max]; max];
    distance[0][0] = 0;
    let mut previous_nodes = vec![vec![(usize::MAX, usize::MAX); max]; max];

    while !visited[max - 1][max - 1] {
        let mut i = usize::MAX;
        let mut j = usize::MAX;
        let mut curr_max = usize::MAX;

        for k in 0..max {
            for l in 0..max {
                if !visited[k][l] && distance[k][l] < curr_max && maze[k][l] != '#' {
                    i = k;
                    j = l;
                    curr_max = distance[k][l];
                }
            }
        }

        update_shortest_path(
            &maze,
            i,
            j,
            &mut distance,
            &mut visited,
            &mut previous_nodes,
        );
    }

    Some(distance[max - 1][max - 1])
}

fn update_shortest_path(
    maze: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    distances: &mut Vec<Vec<usize>>,
    visited: &mut Vec<Vec<bool>>,
    previous_nodes: &mut Vec<Vec<(usize, usize)>>,
) {
    let curr_distance = distances[i][j] + 1;
    if i as i32 - 1 >= 0
        && !visited[i - 1][j]
        && distances[i - 1][j] > curr_distance
        && maze[i - 1][j] != '#'
    {
        distances[i - 1][j] = curr_distance;
        previous_nodes[i - 1][j] = (i, j);
    }
    if j as i32 - 1 >= 0
        && !visited[i][j - 1]
        && distances[i][j - 1] > curr_distance
        && maze[i][j - 1] != '#'
    {
        distances[i][j - 1] = curr_distance;
        previous_nodes[i][j - 1] = (i, j);
    }
    if i + 1 <= maze.len() - 1
        && !visited[i + 1][j]
        && distances[i + 1][j] > curr_distance
        && maze[i + 1][j] != '#'
    {
        distances[i + 1][j] = curr_distance;
        previous_nodes[i + 1][j] = (i, j);
    }

    if j + 1 <= maze.len() - 1
        && !visited[i][j + 1]
        && distances[i][j + 1] > curr_distance
        && maze[i][j + 1] != '#'
    {
        distances[i][j + 1] = curr_distance;
        previous_nodes[i][j + 1] = (i, j);
    }
    visited[i][j] = true;
}

pub fn part_two(input: &str) -> Option<String> {
    let max = 71_usize;
    let starting_point = 12;

    let mut maze = vec![vec!['.'; max]; max];
    let mut blocking_bytes = vec![];
    let lines: Vec<&str> = input.lines().collect();
    let mut lookup = vec![0; lines.len()];
    for i in 0..lines.len() {
        let coordinates: Vec<usize> = lines[i]
            .split(",")
            .map(|point| point.parse::<usize>().unwrap())
            .collect();
        if i < starting_point {
            maze[coordinates[0]][coordinates[1]] = '#';
            lookup[i] = 1;
        }
        blocking_bytes.push((coordinates[0], coordinates[1]));
    }

    get_blocking_byte(&mut maze, &blocking_bytes, 12, lines.len() - 1, &mut lookup);

    for i in 1025..lookup.len() {
        if lookup[i] == 2 && lookup[i - 1] == 1 {
            return Some(to_str(blocking_bytes[i].0, blocking_bytes[i].1));
        }
    }

    None
}

fn get_blocking_byte(
    maze: &mut Vec<Vec<char>>,
    blocking_bytes: &Vec<(usize, usize)>,
    i: usize,
    j: usize,
    lookup: &mut Vec<usize>,
) {
    //get mid point
    let m = i + (j - i) / 2;

    //check if it is in the looup -> if it is then return
    if lookup[m] != 0 {
        return;
    }

    //update the maze accordingly
    for bp in i..=m {
        maze[blocking_bytes[bp].0][blocking_bytes[bp].1] = '#';
    }

    //check if there is a path or not
    let mut visited = vec![vec![false; maze.len()]; maze.len()];
    let flag = is_there_a_path(maze, 0, 0, &mut visited);

    //update the lookup
    lookup[m] = if flag { 1 } else { 2 };

    if flag {
        get_blocking_byte(maze, blocking_bytes, m, j, lookup);
    } else {
        for bp in i..=m {
            maze[blocking_bytes[bp].0][blocking_bytes[bp].1] = '.';
        }
        get_blocking_byte(maze, blocking_bytes, i, m, lookup);
    }
}

fn is_there_a_path(
    maze: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    visited: &mut Vec<Vec<bool>>,
) -> bool {
    if i == maze.len() - 1 && j == maze.len() - 1 {
        return true;
    }

    if maze[i][j] == '#' || visited[i][j] {
        return false;
    }

    visited[i][j] = true;

    if i as i32 - 1 >= 0 && is_there_a_path(maze, i - 1, j, visited) {
        return true;
    }

    if i + 1 <= maze.len() - 1 && is_there_a_path(maze, i + 1, j, visited) {
        return true;
    }

    if j as i32 - 1 >= 0 && is_there_a_path(maze, i, j - 1, visited) {
        return true;
    }

    if j + 1 <= maze.len() - 1 && is_there_a_path(maze, i, j + 1, visited) {
        return true;
    }

    false
}

fn to_str(i: usize, j: usize) -> String {
    format!("{}-{}", i, j)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_shortest_path(&advent_of_code::template::read_file("examples", DAY), 7);
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
