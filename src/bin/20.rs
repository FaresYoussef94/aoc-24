use std::collections::HashMap;

use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

advent_of_code::solution!(20);

#[derive(Debug, Clone)]
struct Map {
    def: Vec<Vec<Point>>,
    start: Point,
    end: Point,
    path: HashMap<Point, usize>,
    visited: Vec<Vec<bool>>,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut def = vec![];
        let path = HashMap::new();
        let mut i = 0;
        let mut j = 0;
        let mut start: Point = Point { i, j, ptype: 'S' };
        let mut end: Point = Point { i, j, ptype: 'E' };
        input.lines().for_each(|line| {
            let mut curr_row = vec![];
            j = 0;
            line.chars().for_each(|ptype| {
                let point = Point { i, j, ptype };
                curr_row.push(point);
                if point.ptype == 'S' {
                    start = point;
                }
                if point.ptype == 'E' {
                    end = point;
                }
                j += 1;
            });
            def.push(curr_row);
            i += 1;
        });
        let visited = vec![vec![false; def.len()]; def.len()];
        Self {
            def,
            start,
            end,
            path,
            visited,
        }
    }

    fn count_cheats_above(&mut self, saved_time: usize, remaining_cheats: usize) -> usize {
        self.traverse(self.start.i, self.start.j, 0);

        let mut sorted_path = self.path.iter().collect::<Vec<_>>();
        sorted_path.sort_by(|a, b| a.1.cmp(b.1));

        sorted_path
            .par_iter()
            .enumerate()
            .map(|(i, (ip, id))| {
                sorted_path
                    .iter()
                    .skip(i + 1)
                    .enumerate()
                    .filter(|(_j, (jp, jd))| {
                        if id >= jd {
                            return false;
                        }

                        let distance = *jd - *id;
                        let manhattan_distance = ip.i.abs_diff(jp.i) + ip.j.abs_diff(jp.j);

                        manhattan_distance <= remaining_cheats
                            && distance - manhattan_distance >= saved_time
                    })
                    .count()
            })
            .sum()
    }

    fn traverse(&mut self, ci: usize, cj: usize, score: usize) -> bool {
        if self.visited[ci][cj] || self.def[ci][cj].ptype == '#' {
            return false;
        }

        if self.end.i == ci && self.end.j == cj {
            self.path.insert(self.def[ci][cj], score);
            return true;
        }

        self.visited[ci][cj] = true;

        if (ci >= 1 && self.traverse(ci - 1, cj, score + 1))
            || (cj >= 1 && self.traverse(ci, cj - 1, score + 1))
            || (ci <= self.def.len() - 2 && self.traverse(ci + 1, cj, score + 1))
            || (cj <= self.def.len() - 2 && self.traverse(ci, cj + 1, score + 1))
        {
            self.path.insert(self.def[ci][cj], score);
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    i: usize,
    j: usize,
    ptype: char,
}

impl Point {}

pub fn part_one(input: &str) -> Option<usize> {
    solve(input, 100, 2)
}

pub fn part_two(input: &str) -> Option<usize> {
    solve(input, 100, 20)
}

fn solve(input: &str, max: usize, remaining_cheats: usize) -> Option<usize> {
    let mut map = Map::new(input);
    Some(map.count_cheats_above(max, remaining_cheats))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY), 12, 2);
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = solve(
            &advent_of_code::template::read_file("examples", DAY),
            50,
            20,
        );
        assert_eq!(result, Some(285));
    }
}
