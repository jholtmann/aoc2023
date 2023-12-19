use clap::Parser;
use std::collections::HashMap;
use std::fs;
use aoc2023::Args;
use aoc2023::utils::read_int;

fn main() -> anyhow::Result<()> {
    let marble_limits: HashMap<&str, usize> =
        HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let args = Args::parse();

    let contents = fs::read_to_string(args.input_file)?;
    let lines = contents.lines();

    let mut solution = 0;

    for line in lines {
        let content = String::from(line);

        // extract game number
        let game_index = content.find(":").unwrap();
        let game = &content[..game_index]
            .replace("Game ", "")
            .parse::<usize>()?;

        // split game into component hands
        let hands = &content[(game_index + 2)..]
            .split("; ")
            .collect::<Vec<&str>>();

        // track if game is still valid
        let mut allowed_game = true;
        for hand in hands {
            // extract marble counts from hand
            let mut hand_marble_counts = HashMap::new();
            for marble_data in &hand.split(", ").collect::<Vec<&str>>() {
                let (marble_count, i) = read_int(marble_data)?;
                let color = &marble_data[(i + 1)..];

                if hand_marble_counts.contains_key(color) {
                    *hand_marble_counts.get_mut(&color).unwrap() += marble_count;
                } else {
                    hand_marble_counts.insert(color, marble_count);
                }
            }

            // test if hand is valid
            for (color, marble_total) in hand_marble_counts {
                if !marble_limits.contains_key(color) {
                    panic!("invalid color {color}");
                }

                let limit = marble_limits[color];

                if marble_total > limit {
                    allowed_game = false;
                    println!("game {game} {color} {marble_total} > {limit}");
                    break;
                }
            }

            // if a hand is invalid, there's no need to keep parsing the rest of the hands
            if !allowed_game {
                break;
            }
        }

        if allowed_game {
            solution += game;
        }
    }

    println!("Result: {}", solution);

    Ok(())
}
