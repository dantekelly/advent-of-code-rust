use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[derive(Clone, Hash, Eq, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

fn solve_challenge(input: String, desired: (i32, i32, i32)) -> i32 {
    let game_regex = Regex::new(r"(?m)^Game (\d+): (.+)$").unwrap();

    // Get all games and parse them into a hashmap
    let games: Vec<(i32, Vec<HashMap<Color, i32>>)> = game_regex
        .captures_iter(input.as_str())
        .map(|game_capture| {
            let (_, [game_number, game_contents]) = game_capture.extract();
            let game_bags: Vec<HashMap<Color, i32>> = game_contents
                .split_terminator("; ")
                .map(|game_contents| {
                    let mut bag: HashMap<Color, i32> = HashMap::new();

                    game_contents.split(", ").for_each(|bag_item| {
                        let split_item: Vec<&str> = bag_item.split(" ").collect();
                        let count = split_item[0].parse::<i32>().unwrap();
                        let color = match split_item[1] {
                            "red" => Color::Red,
                            "green" => Color::Green,
                            "blue" => Color::Blue,
                            _ => panic!("Invalid color"),
                        };

                        bag
                            .entry(color)
                            .and_modify(|bag_count| *bag_count = (*bag_count).max(count))
                            .or_insert(count);
                    });

                    bag
                })
                .collect();

            (game_number.parse::<i32>().unwrap(), game_bags)
        })
        .collect();

    let greedy_games: Vec<(i32, Vec<HashMap<Color, i32>>)> = games
        .iter()
        .filter(|(_, hm)| {
            hm.iter().all(|bag| {
                bag.iter().all(|(color, &val)| {
                    match color {
                        Color::Red => val <= desired.0,
                        Color::Green => val <= desired.1,
                        Color::Blue => val <= desired.2,
                    }
                })
            })
        })
        .cloned()
        .collect();

    greedy_games
        .into_iter()
        .map(|(id, _)| id)
        .sum()
}

fn main() {
    let input = fs::read_to_string("./src/bin/input.txt").unwrap();

    // 2176

    println!("Answer: {}", solve_challenge(input, (12, 13, 14)));
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

        assert_eq!(solve_challenge(input, (12, 13, 14)), 8)
    }
}