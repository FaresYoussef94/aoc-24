use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::Bytes;
use std::iter::once;
use std::mem;

advent_of_code::solution!(25);

pub fn part_one(input: &str) -> Option<u32> {
    let x: Vec<_> = input
        .split("\n\n")
        .map(|e| {
            let yx = e.lines().map(|x| x.bytes().collect_vec()).collect_vec();
            let is_key = yx[0][0] == b'.';
            (
                is_key,
                (0..5usize)
                    .map(|x| (1..6usize).filter(|&y| yx[y][x] == b'#').count())
                    .collect_vec(),
            )
        })
        .collect_vec();
    let keys = x.iter().filter(|(k, x)| *k).map(|(k, x)| x).collect_vec();
    let locks = x.iter().filter(|(k, x)| !*k).map(|(k, x)| x).collect_vec();
    let mut cnt = 0;
    for k in keys {
        for l in &locks {
            if k.iter().zip(*l).all(|(&k, &l)| k + l <= 5) {
                cnt += 1;
            }
        }
    }
    Some(cnt)
}

pub fn part_two(_input: &str) -> Option<String> {
    Some("Merry Christmas".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert!(result.is_some());
    }
}
