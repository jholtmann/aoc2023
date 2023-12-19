use clap::Parser;
use std::fs;
use aoc2023::Args;
use aoc2023::utils::read_ints;


fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let contents = fs::read_to_string(args.input_file)?;
    let lines = contents.lines();

    let mut solution = 0;

    for line in lines {
        let line = &line[line.find(':').unwrap()+2..];
        let line = line.split('|').collect::<Vec<&str>>();

        let target = read_ints(line[0], ' ');
        let hand = read_ints(line[1], ' ');

        println!("{:?}", target);
        println!("{:?}", hand);

        let mut hand_value = 0;
        for value in hand {
            if !target.contains(&value) {
                continue;
            }

            if hand_value == 0 {
                hand_value = 1;
            } else {
                hand_value *= 2;
            }
        }

        solution += hand_value;
    }

    println!("Result: {}", solution);

    Ok(())
}
