use std::ops::RangeInclusive;

advent_of_code::solution!(5);

fn parse_range(range_str: &str) -> Option<RangeInclusive<u64>> {
    let (s, e) = range_str.split_once("-")?;
    let start = s.trim().parse::<u64>().ok()?;
    let end = e.trim().parse::<u64>().ok()?;
    Some(start..=end)
}

pub fn part_one(input: &str) -> Option<u64> {
    let parts = input.split_once("\n\n").unwrap();
    let ranges: Vec<_> = parts.0.lines().filter_map(parse_range).collect();
    let fresh = parts
        .1
        .lines()
        .filter_map(|l| l.trim().parse::<u64>().ok())
        .filter(|f| ranges.iter().any(|r| r.contains(f)))
        .count();

    Some(fresh as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let parts = input.trim().split_once("\n\n").unwrap();
    let mut ranges: Vec<_> = parts.0.lines().filter_map(parse_range).collect();
    ranges.sort_by(|a, b| a.start().cmp(b.start()));

    let mut merged = vec![ranges[0].clone()];
    for range in ranges.iter().skip(1) {
        let last = merged.last_mut().unwrap();
        if *range.start() <= last.end() + 1 {
            *last = *last.start()..=(*last.end()).max(*range.end());
        } else {
            merged.push(range.clone());
        }
    }
    let count: u64 = merged.iter().map(|r| r.end() - r.start() + 1).sum();
    Some(count)
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
