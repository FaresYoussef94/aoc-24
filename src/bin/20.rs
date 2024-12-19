use std::collections::{HashMap, HashSet};

advent_of_code::solution!(20);

#[derive(Debug, Clone)]
struct Map {
    def: Vec<Vec<Point>>,
    start: Point,
    end: Point,
    path: HashMap<Point, usize>,
    visited: Vec<Vec<bool>>,
    shortcuts: HashMap<String, usize>,
    visited_shortcuts: HashSet<String>,
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
        let shortcuts = HashMap::new();
        let visited_shortcuts = HashSet::new();
        Self {
            def,
            start,
            end,
            path,
            visited,
            shortcuts,
            visited_shortcuts,
        }
    }

    fn count_cheats_above(&mut self, saved_time: usize, remaining_cheats: usize) -> usize {
        self.traverse(self.start.i, self.start.j, 0);

        let path_keys: Vec<Point> = self.path.keys().cloned().collect();

        for point in path_keys {
            let i = point.i;
            let j = point.j;

            self.update_cheats_score(i, j, i - 1, j, remaining_cheats - 1);
            self.update_cheats_score(i, j, i + 1, j, remaining_cheats - 1);
            self.update_cheats_score(i, j, i, j - 1, remaining_cheats - 1);
            self.update_cheats_score(i, j, i, j + 1, remaining_cheats - 1);
        }

        let mut lookup: HashMap<usize, usize> = HashMap::new();

        for (_key, val) in self.shortcuts.iter() {
            lookup.entry(*val).and_modify(|os| *os += 1).or_insert(1);
        }

        let mut sorted: Vec<_> = lookup.iter().collect();
        sorted.sort_by(|a, b| a.0.cmp(b.0).then_with(|| b.1.cmp(a.1)));

        for (key, value) in sorted {
            if *key >= 50 {
                println!("There are {} with value {}", value, key);
            }
        }

        lookup
            .iter()
            .filter(|(k, _v)| **k >= saved_time)
            .map(|(_k, v)| v)
            .sum::<usize>()
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

    fn update_cheats_score(
        &mut self,
        i: usize,
        j: usize,
        wi: usize,
        wj: usize,
        remaining_cheats: usize,
    ) {
        if self.def[wi][wj].ptype != '#' || remaining_cheats == 0 {
            return;
        }

        if self
            .visited_shortcuts
            .contains(&format!("{}-{}-{}-{}", i, j, wi, wj).to_string())
        {
            return;
        }

        self.visited_shortcuts
            .insert(format!("{}-{}-{}-{}", i, j, wi, wj));

        let max = self.def.len() - 1;

        // check up
        if wi > 0 && wi - 1 != i {
            let pi = wi - 1;
            let pj = wj;
            self.comp_and_update(i, j, pi, pj);
            self.update_cheats_score(i, j, pi, pj, remaining_cheats - 1);
        }
        // check down
        if wi < max && wi + 1 != i {
            let pi = wi + 1;
            let pj = wj;
            self.comp_and_update(i, j, pi, pj);
            self.update_cheats_score(i, j, pi, pj, remaining_cheats - 1);
        }
        // check left
        if wj > 0 && wj - 1 != j {
            let pi = wi;
            let pj = wj - 1;
            self.comp_and_update(i, j, pi, pj);
            self.update_cheats_score(i, j, pi, pj, remaining_cheats - 1);
        }
        // check right
        if wj < max && wj + 1 != j {
            let pi = wi;
            let pj = wj + 1;
            self.comp_and_update(i, j, pi, pj);
            self.update_cheats_score(i, j, pi, pj, remaining_cheats - 1);
        }
    }

    fn comp_and_update(&mut self, i: usize, j: usize, pi: usize, pj: usize) {
        if !self.path.contains_key(&self.def[pi][pj]) {
            return;
        }

        let cheat_key = format!("{}-{}->{}-{}", i, j, pi, pj);
        let score = self.path.get(&self.def[i][j]).unwrap();
        let pscore = self.path.get(&self.def[pi][pj]).unwrap();
        let si = self.def[i][j].i;
        let sj = self.def[i][j].j;
        let ei = self.def[pi][pj].i;
        let ej = self.def[pi][pj].j;
        let abs_diff = si.abs_diff(ei) + sj.abs_diff(ej);

        if *pscore <= score + abs_diff {
            return;
        }

        self.shortcuts
            .entry(cheat_key)
            .and_modify(|os| *os = (*os).max(*pscore - score - abs_diff))
            .or_insert(pscore - score - abs_diff);
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
    solve(input, 100, 19)
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
