advent_of_code::solution!(2);

fn parse_range(range_str: &str) -> Option<impl Iterator<Item = u64>> {
    let (s, e) = range_str.split_once("-")?;
    let start = s.trim().parse::<u64>().ok()?;
    let end = e.trim().parse::<u64>().ok()?;
    Some(start..=end)
}

fn is_repeating(s: &[u8], sl: usize) -> bool {
    let pattern = &s[..sl];
    s.chunks(sl).skip(1).all(|c| c == pattern)
}

fn has_repeating_pattern(id: &u64) -> bool {
    let s = id.to_string().into_bytes();
    let len = s.len();
    (1..=(len / 2))
        .filter(|&sl| len.is_multiple_of(sl))
        .any(|sl| is_repeating(&s, sl))
}

fn repeats_twice(id: &u64) -> bool {
    let s = id.to_string().into_bytes();
    let len = s.len();
    len.is_multiple_of(2) && s[..(len / 2)] == s[(len / 2)..]
}

pub fn part_one(input: &str) -> Option<u64> {
    input
        .split(",")
        .filter_map(parse_range)
        .flat_map(|range| range.filter(repeats_twice))
        .sum::<u64>()
        .into()
}

pub fn part_two(input: &str) -> Option<u64> {
    input
        .split(",")
        .filter_map(parse_range)
        .flat_map(|range| range.filter(has_repeating_pattern))
        .sum::<u64>()
        .into()
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
