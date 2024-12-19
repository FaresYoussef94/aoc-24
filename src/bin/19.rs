use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

advent_of_code::solution!(19);

#[derive(Debug)]
struct Patterns {
    patterns: Vec<Pattern>,
}

impl Patterns {
    fn new(input: &str) -> Self {
        let patterns: Vec<Pattern> = input
            .split(",")
            .filter(|a| !a.is_empty())
            .map(|a| a.trim())
            .map(Pattern::new)
            .collect();

        Self { patterns }
    }

    fn count_possible_designs(&self, designs: Designs) -> usize {
        let lookup = Arc::new(Mutex::new(HashMap::new()));
        let lookup_clone = Arc::clone(&lookup);

        designs
            .designs
            .par_iter()
            .map(|design| {
                let local_lookup = Arc::clone(&lookup_clone);

                // Lock the mutex when accessing the hashmap
                let mut lookup_guard = local_lookup.lock().unwrap();

                if self.is_design_possible(design, 0, true, &mut lookup_guard) > 0 {
                    1
                } else {
                    0
                }
            })
            .sum()
    }

    fn count_all_possible_designs(&self, designs: Designs) -> usize {
        let lookup = Arc::new(Mutex::new(HashMap::new()));
        let lookup_clone = Arc::clone(&lookup);

        designs
            .designs
            .par_iter()
            .map(|design| {
                let local_lookup = Arc::clone(&lookup_clone);

                // Lock the mutex when accessing the hashmap
                let mut lookup_guard = local_lookup.lock().unwrap();
                self.is_design_possible(design, 0, false, &mut lookup_guard)
            })
            .sum()
    }

    fn is_design_possible(
        &self,
        design: &Design,
        i: usize,
        early_break: bool,
        lookup: &mut HashMap<String, usize>,
    ) -> usize {
        if lookup.contains_key(&design.design[i..].iter().collect::<String>()) {
            return *lookup
                .get(&design.design[i..].iter().collect::<String>())
                .unwrap();
        }
        if i == design.design.len() {
            return 1;
        }

        let mut poss_ways = 0;
        for pattern in self.patterns.iter() {
            match pattern.traverse(&design.design.clone(), i) {
                Some(ni) => {
                    poss_ways += self.is_design_possible(design, ni, early_break, lookup);
                }
                None => {
                    continue;
                }
            }

            if early_break && poss_ways >= 1 {
                break;
            }
        }
        lookup.insert(design.design[i..].iter().collect(), poss_ways);
        poss_ways
    }
}

#[derive(Debug, Clone)]
struct Pattern {
    def: Vec<char>,
}

impl Pattern {
    fn new(input: &str) -> Self {
        Self {
            def: input.chars().collect(),
        }
    }

    fn traverse(&self, input: &Vec<char>, i: usize) -> Option<usize> {
        if self.def.len() + i > input.len() {
            return None;
        }

        for j in 0..self.def.len() {
            if self.def[j] != input[i + j] {
                return None;
            }
        }

        Some(i + self.def.len())
    }
}

#[derive(Debug)]
struct Designs {
    designs: Vec<Design>,
}

impl Designs {
    fn new(input: &str) -> Self {
        let designs: Vec<Design> = input.split_whitespace().map(Design::new).collect();
        Self { designs }
    }
}

#[derive(Debug)]
struct Design {
    design: Vec<char>,
}

impl Design {
    fn new(input: &str) -> Self {
        Self {
            design: input.chars().collect(),
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let patterns = Patterns::new(parts[0]);
    let designs = Designs::new(parts[1]);

    Some(patterns.count_possible_designs(designs))
}

pub fn part_two(input: &str) -> Option<usize> {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let patterns = Patterns::new(parts[0]);
    let designs = Designs::new(parts[1]);

    Some(patterns.count_all_possible_designs(designs))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
