advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let vec: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let mut result: u32 = 0;

    for i in 0..vec.len() {
        for j in 0..vec[i].len() {
            if vec[i][j] == 'X' {
                // Right
                if i + 3 < vec.len()
                    && vec[i + 1][j] == 'M'
                    && vec[i + 2][j] == 'A'
                    && vec[i + 3][j] == 'S'
                {
                    result += 1;
                }

                // Left
                if i >= 3 && vec[i - 1][j] == 'M' && vec[i - 2][j] == 'A' && vec[i - 3][j] == 'S' {
                    result += 1;
                }

                // Up
                if j >= 3 && vec[i][j - 1] == 'M' && vec[i][j - 2] == 'A' && vec[i][j - 3] == 'S' {
                    result += 1;
                }

                // Down
                if j + 3 < vec[i].len()
                    && vec[i][j + 1] == 'M'
                    && vec[i][j + 2] == 'A'
                    && vec[i][j + 3] == 'S'
                {
                    result += 1;
                }

                // Diagonal up right
                if i + 3 < vec.len()
                    && j >= 3
                    && vec[i + 1][j - 1] == 'M'
                    && vec[i + 2][j - 2] == 'A'
                    && vec[i + 3][j - 3] == 'S'
                {
                    result += 1;
                }

                // Diagonal up left
                if i >= 3
                    && j >= 3
                    && vec[i - 1][j - 1] == 'M'
                    && vec[i - 2][j - 2] == 'A'
                    && vec[i - 3][j - 3] == 'S'
                {
                    result += 1;
                }

                // Diagonal down right
                if i + 3 < vec.len()
                    && j + 3 < vec[i].len()
                    && vec[i + 1][j + 1] == 'M'
                    && vec[i + 2][j + 2] == 'A'
                    && vec[i + 3][j + 3] == 'S'
                {
                    result += 1;
                }

                // Diagonal down left
                if i >= 3
                    && j + 3 < vec[i].len()
                    && vec[i - 1][j + 1] == 'M'
                    && vec[i - 2][j + 2] == 'A'
                    && vec[i - 3][j + 3] == 'S'
                {
                    result += 1;
                }
            }
        }
    }

    Some(result)
}
pub fn part_two(input: &str) -> Option<u32> {
    let vec: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let mut result: u32 = 0;

    for i in 0..vec.len() {
        for j in 0..vec[i].len() {
            if vec[i][j] == 'A' && i >= 1 && i + 1 < vec.len() && j >= 1 && j + 1 < vec.len() {
                let mut orien_b: usize = 0;
                if vec[i - 1][j - 1] == 'M' && vec[i + 1][j + 1] == 'S' {
                    orien_b += 1;
                }
                if vec[i + 1][j + 1] == 'M' && vec[i - 1][j - 1] == 'S' {
                    orien_b += 1;
                }
                if vec[i - 1][j + 1] == 'M' && vec[i + 1][j - 1] == 'S' {
                    orien_b += 1;
                }
                if vec[i + 1][j - 1] == 'M' && vec[i - 1][j + 1] == 'S' {
                    orien_b += 1;
                }

                if orien_b == 2 {
                    result += 1;
                }
            }
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
