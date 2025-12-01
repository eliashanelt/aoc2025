use aoc2025::read_input;

fn part1(input: &str) -> i32 {
    let mut dial_pos = 50;
    let mut stop_at_zero_cnt = 0;
    for l in input.lines() {
        let step = l[1..].parse::<i32>().unwrap();
        let dir = match l.chars().next().unwrap() {
            'L' => -1,
            'R' => 1,
            c => panic!("undefined char: {c}"),
        };
        let new_pos = dial_pos + dir * step;
        dial_pos = new_pos.rem_euclid(100);
        if dial_pos == 0 {
            stop_at_zero_cnt += 1;
        }
    }
    stop_at_zero_cnt
}

fn part2(input: &str) -> i32 {
    let mut zero_crossing_cnt = 0;
    let mut dial_pos = 50;
    for l in input.lines() {
        let step = l[1..].parse::<i32>().unwrap();
        let dir = match l.chars().next().unwrap() {
            'L' => -1,
            'R' => 1,
            c => panic!("undefined char: {c}"),
        };
        let new_pos = dial_pos + dir * step;
        zero_crossing_cnt += if dir == 1 {
            new_pos.div_euclid(100) - dial_pos.div_euclid(100)
        } else {
            (dial_pos - 1).div_euclid(100) - (new_pos - 1).div_euclid(100)
        };
        dial_pos = new_pos.rem_euclid(100);
    }
    zero_crossing_cnt
}

fn main() {
    let input = read_input(1);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
    "#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 0);
    }
}
