use clap::Parser;
use std::fs;
use aoc2023::Args;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let contents = fs::read_to_string(args.input_file)?;
    let lines = contents.lines();

    let mut sum: u64 = 0;
    for line in lines {
        let mut calib_value = String::from("");

        for c in line.chars() {
            if !c.is_numeric() {
                continue;
            }

            calib_value.push(c);
            break;
        }

        for c in line.chars().rev() {
            if !c.is_numeric() {
                continue;
            }

            calib_value.push(c);
            break;
        }

        sum += calib_value.parse::<u64>()?;
    }

    println!("Calibration sum: {}", sum);

    Ok(())
}
