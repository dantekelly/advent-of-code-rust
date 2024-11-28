use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

fn solve_challenge(input: String) -> i32 {
    let game_regex = Regex::new(r"(?m)^Game (\d+): (.+)$").unwrap();

    game_regex.captures_iter(input.as_str()).map(|game_capture| {
        let (_, [game_number, game_contents]) = game_capture.extract();
        let mut bag: HashMap<Color, i32> = HashMap::new();

        game_contents.split_terminator("; ").for_each(|game_contents| {
            game_contents.split_terminator(", ").for_each(|bag_item| {
                let split_item: Vec<&str> = bag_item.split_terminator(" ").collect();
                let count = split_item[0].parse::<i32>().unwrap_or(0);
                let color = match split_item[1] {
                    "red" => Color::Red,
                    "green" => Color::Green,
                    "blue" => Color::Blue,
                    _ => panic!("Invalid color"),
                };

                bag.entry(color).and_modify(|bag_count| {
                    if *bag_count < count {
                        *bag_count = count;
                    }
                }).or_insert(count);
            });
        });

        println!("{:?}", bag);

        bag.values().product::<i32>()
    }).sum()
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
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string();

        assert_eq!(solve_challenge(input), 2286)
    }
}