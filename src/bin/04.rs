advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let map: Vec<Vec<bool>> = input
        .trim()
        .lines()
        .map(|l| l.chars().map(|c| c == '@').collect())
        .collect();
    let mut cnt = 0;
    for y in 0..(map.len() as i32) {
        for x in 0..(map[0].len() as i32) {
            if map[y as usize][x as usize] {
                let mut adj_cnt = 0;
                for dir in DIRS {
                    if x + dir.0 >= 0
                        && x + dir.0 < map[0].len() as i32
                        && y + dir.1 >= 0
                        && y + dir.1 < map.len() as i32
                        && map[(y + dir.1) as usize][(x + dir.0) as usize]
                    {
                        adj_cnt += 1;
                    }
                }
                if adj_cnt < 4 {
                    cnt += 1
                }
            }
        }
    }
    dbg!(Some(cnt))
}

const DIRS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn part_two(input: &str) -> Option<u64> {
    let mut map: Vec<Vec<bool>> = input
        .trim()
        .lines()
        .map(|l| l.chars().map(|c| c == '@').collect())
        .collect();
    let mut removed = true;
    let mut cnt = 0;
    while removed {
        removed = false;
        for y in 0..(map.len() as i32) {
            for x in 0..(map[0].len() as i32) {
                if map[y as usize][x as usize] {
                    let mut adj_cnt = 0;
                    for dir in DIRS {
                        if x + dir.0 >= 0
                            && x + dir.0 < map[0].len() as i32
                            && y + dir.1 >= 0
                            && y + dir.1 < map.len() as i32
                            && map[(y + dir.1) as usize][(x + dir.0) as usize]
                        {
                            adj_cnt += 1;
                        }
                    }
                    if adj_cnt < 4 {
                        removed = true;
                        map[y as usize][x as usize] = false;
                        cnt += 1;
                    }
                }
            }
        }
    }
    Some(cnt)
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
