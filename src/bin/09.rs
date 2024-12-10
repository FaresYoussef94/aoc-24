use std::{char, cmp::min, usize, vec};

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let disk_map: Vec<usize> = input
        .split("")
        .filter(|x| !x.is_empty())
        .flat_map(|x| x.parse::<usize>())
        .collect();
    let mut raw_disk: Vec<String> = Vec::new();

    for (i, block) in disk_map.iter().enumerate() {
        for j in 0..*block {
            let id = (i / 2).to_string();
            if i % 2 == 0 {
                raw_disk.push(id);
            } else {
                raw_disk.push(".".to_string());
            }
        }
    }

    let mut i: usize = 0;
    let mut j: usize = raw_disk.len() - 1;

    while i < j {
        if raw_disk[i] == "." && raw_disk[j] != "." {
            raw_disk[i] = raw_disk[j].clone();
            raw_disk[j] = ".".to_string();
            i += 1;
            j -= 1;
        } else if raw_disk[i] != "." && raw_disk[j] == "." {
            i += 1;
            j -= 1;
        } else if raw_disk[i] != "." && raw_disk[j] != "." {
            i += 1;
        } else {
            j -= 1;
        }
    }

    let mut result: u64 = 0;
    let mut k: usize = 0;

    while raw_disk[k] != "." {
        let curr_val = raw_disk[k].parse::<u64>().unwrap();
        let pos = k as u64;
        result += curr_val * pos;
        k += 1;
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let disk_map: Vec<usize> = input
        .split("")
        .filter(|x| !x.is_empty())
        .flat_map(|x| x.parse::<usize>())
        .collect();
    let mut raw_disk: Vec<String> = Vec::new();

    for (i, block) in disk_map.iter().enumerate() {
        for j in 0..*block {
            let id = (i / 2).to_string();
            if i % 2 == 0 {
                raw_disk.push(id);
            } else {
                raw_disk.push(".".to_string());
            }
        }
    }
    // println!("Disk: {:?}", raw_disk);

    let mut i: usize = raw_disk.len() - 1;

    while i > 0 {
        if raw_disk[i] == "." {
            i -= 1;
            continue;
        }

        let mut k = i;

        while k > 0 && raw_disk[k] == raw_disk[i] {
            k -= 1;
        }

        // println!("found num {} range {}-{}  ", raw_disk[i], i, k);

        let mut j = 0;
        while j < i {
            if raw_disk[j] != "." {
                j += 1;
                continue;
            }

            // println!("Checking {}", raw_disk[j]);

            let mut l = j;

            while raw_disk[l] == raw_disk[j] {
                l += 1;
            }

            // println!("Found dots range {}-{}", l, j);

            if (i - k) <= (l - j) {
                // println!("Num range fitting");
                let m = min(i - k, l - j);
                for n in 0..m {
                    raw_disk[j + n] = String::from(raw_disk[i - n].clone());
                    raw_disk[i - n] = ".".to_string();
                }
                // println!("Raw disk {:?}", raw_disk);
                break;
            } else {
                // println!("num range is bigger checking the next");
            }
            j += 1;
        }
        i = k;
    }

    let mut result: u64 = 0;
    let mut k: usize = 0;

    // println!("RawDis: {:?}", raw_disk);

    while k < raw_disk.len() {
        if raw_disk[k] == "." {
            k += 1;
            continue;
        }
        let curr_val = raw_disk[k].parse::<u64>().unwrap();
        let pos = k as u64;
        result += curr_val * pos;
        k += 1;
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
