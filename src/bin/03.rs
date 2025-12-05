advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    p(input, 2).into()
}

pub fn part_two(input: &str) -> Option<u64> {
    p(input, 12).into()
}

fn r(len_left: u64, digits: &[u64]) -> u64 {
    if len_left == 0 {
        return 0;
    }
    let end = digits.len() - len_left as usize;
    let (max_index, &max_digit) = digits[..=end]
        .iter()
        .enumerate()
        .rev()
        .max_by_key(|&(_, &d)| d)
        .unwrap();

    max_digit * 10u64.pow(len_left as u32 - 1) + r(len_left - 1, &digits[max_index + 1..])
}

fn p(input: &str, len: u64) -> u64 {
    input
        .lines()
        .map(|l| {
            let digits: Vec<u64> = l
                .chars()
                .filter_map(|d| d.to_digit(10).map(u64::from))
                .collect();
            r(len, &digits)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
