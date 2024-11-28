use regex::Regex;
use std::collections::HashMap;
use std::fs;

// Game 1: 4 green, 2 blue; 1 red, 1 blue, 4 green; 3 green, 4 blue, 1 red; 7 green, 2 blue, 4 red; 3 red, 7 green; 3 red, 3 green

// 4 green, 2 blue; 1 red, 1 blue, 4 green; 3 green, 4 blue, 1 red; 7 green, 2 blue, 4 red; 3 red, 7 green; 3 red, 3 green

fn solve_challenge(input: String, desired: (i32, i32, i32)) -> i32 {
    let game_regex = Regex::new(r"(?m)^Game (\d+): (.+)$").unwrap();

    // Get all games and parse them into a hashmap
    let games: Vec<(i32, Vec<HashMap<&str, i32>>)> = game_regex.captures_iter(input.as_str()).map(|c| {
        let (_, [game_number, game_contents]) = c.extract();
        let mut game_bags: Vec<HashMap<&str, i32>> = Vec::new();

        game_contents.split_terminator("; ").for_each(|x| {
            let mut bag: HashMap<&str, i32> = HashMap::new();
            x.split_terminator(", ").for_each(|bag_item| {
                let split_item: Vec<&str> = bag_item.split_terminator(" ").collect();
                let count: i32 = split_item[0].parse().unwrap_or(0);
                let color = split_item[1];

                bag.entry(color).and_modify(|e| {
                    if *e > count {
                        *e = count;
                    }
                }).or_insert(count);
            });

            game_bags.push(bag);
        });

        (game_number.parse::<i32>().unwrap(), game_bags)
    }).collect();

    let greedy_games: Vec<(i32, Vec<HashMap<&str, i32>>)> = games.iter().filter(|(_, hm)| {
        hm.iter().all(|bag| {
            bag.iter().all(|(&key, &val)| {
                if key == "red" && val <= desired.0 {
                    true
                } else if key == "green" && val <= desired.1 {
                    true
                } else if key == "blue" && val <= desired.2 {
                    true
                } else {
                    false
                }
            })
        })
    }).cloned().collect();
    return greedy_games.into_iter().map(|e| e.0).sum();
}

fn main() {
    let input = fs::read_to_string("./src/bin/input.txt").unwrap();

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
        // assert_eq!(solve_challenge(input, (12, 13, 14)), 2286)
    }
}