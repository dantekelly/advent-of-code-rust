use std::fs;

fn solve_challenge(input: String) -> i32 {
    let mut first_col: Vec<i32> = vec![];
    let mut second_col: Vec<i32> = vec![];

    input.lines().for_each(|line| {
        let cols: Vec<&str> = line.split_whitespace().collect();

        first_col.push(cols[0].to_string().parse::<i32>().unwrap());
        second_col.push(cols[1].to_string().parse::<i32>().unwrap());
    });

    first_col.sort();
    second_col.sort();

    let mut index = 0;
    let mut solution = 0;
    while index < first_col.len() {
        let first_item = first_col[index];
        let second_item = second_col[index];

        solution += first_item.max(second_item) - first_item.min(second_item);
        index += 1;
    }

    return solution;
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
