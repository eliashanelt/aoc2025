pub fn read_input(day: u32) -> String {
    let path = format!("inputs/day{:02}.txt", day);
    std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("Could not read input file: {}", path))
}

pub fn parse_lines<T>(input: &str) -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().expect("Failed to parse line"))
        .collect()
}

pub fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

pub fn split_sections(input: &str) -> Vec<&str> {
    input.split("\n\n").collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_lines() {
        let input = "1\n2\n3\n";
        let nums: Vec<i32> = parse_lines(input);
        assert_eq!(nums, vec![1, 2, 3]);
    }
}
