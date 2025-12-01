#!/bin/bash

# Load environment variables from .env file if it exists
if [ -f .env ]; then
    export $(cat .env | grep -v '^#' | xargs)
fi

if [ -z "$1" ]; then
    echo "Usage: ./new_day.sh <day_number>"
    exit 1
fi

DAY=$(printf "%02d" $1)

# Create the day file
cat > src/days/day${DAY}.rs << 'EOF'
use aoc2025::read_input;

fn part1(input: &str) -> i32 {
    // TODO: Implement part 1
    0
}

fn part2(input: &str) -> i32 {
    // TODO: Implement part 2
    0
}

fn main() {
    let input = read_input(DAY_NUM);
    
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"
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
EOF

# Replace DAY_NUM with actual number
sed -i "s/DAY_NUM/$1/" src/days/day${DAY}.rs

# Add to Cargo.toml
echo "" >> Cargo.toml
echo "[[bin]]" >> Cargo.toml
echo "name = \"day${DAY}\"" >> Cargo.toml
echo "path = \"src/days/day${DAY}.rs\"" >> Cargo.toml

echo "Created day ${DAY}!"
echo "Fetching input..."
cargo run --bin aoc fetch $1

echo ""
echo "Ready! Edit src/days/day${DAY}.rs and run with: cargo run --bin day${DAY}"
