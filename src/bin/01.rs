advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
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
    Some(stop_at_zero_cnt)
}

pub fn part_two(input: &str) -> Option<u64> {
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
    Some(zero_crossing_cnt as u64)
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
