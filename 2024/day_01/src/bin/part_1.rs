use std::fs;

fn solve_challenge(input: String) -> i32 {
    let (mut col1, mut col2): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            let mut cols = line
                .split_whitespace()
                .map(|n| n.to_string().parse::<i32>().unwrap());

            (cols.next().unwrap(), cols.next().unwrap())
        })
        .unzip();

    col1.sort();
    col2.sort();

    col1.iter()
        .zip(col2.iter())
        .map(|(a, b)| a.max(b) - a.min(b))
        .sum()
}

fn main() {
    let input = fs::read_to_string("./src/bin/input.txt").unwrap();

    println!("Answer: {}", solve_challenge(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3"
            .to_string();

        assert_eq!(solve_challenge(input), 11)
    }
}
