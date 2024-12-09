use regex::Regex;
use std::fs;

fn solve_challenge(input: &str) -> u32 {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    input
        .lines()
        .flat_map(|line| {
            re.captures_iter(line).map(|c| {
                let [n1, n2] = c.extract::<2>().1;
                n1.parse::<u32>().unwrap() * n2.parse::<u32>().unwrap()
            })
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("./src/bin/input.txt").unwrap();

    println!("Answer: {}", solve_challenge(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(solve_challenge(input), 161)
    }
}
