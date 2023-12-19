use clap::Parser;
use std::fs;
use aoc2023::Args;
use aoc2023::utils::RADIX;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let contents = fs::read_to_string(args.input_file)?;
    let mut lines = contents.lines();

    let mut solution = 0;

    let period = char::from('.');

    // parse first line to figure out x length of input
    let first_line = lines.next().unwrap();
    let mut line_data = Vec::new();
    line_data.push(period);
    line_data.extend(first_line.chars());
    line_data.push(period);

    // don't want to consume lines to figure out the y size for capacity constructor
    let mut data = Vec::new();

    // pad vector to make future processing easier
    let line_len = line_data.len();
    data.push(vec![period; line_len]);
    data.push(line_data);

    // read file into array
    for line in lines {
        let mut line_data = Vec::new();

        // pad vector to make future parsing easier
        line_data.push(period);
        line_data.extend(line.chars());
        line_data.push(period);

        data.push(line_data);
    }

    data.push(vec![period; line_len]);

    // println!("Data: {:?}", data);

    // loop over 2D array and determine which values to count
    for r in 1..data.len() - 1 {
        let mut number = String::new();
        let mut adjacent = false;

        for c in 1..line_len {
            let val = &data[r][c];
            if !val.is_digit(RADIX) {
                if number.len() > 0 {
                    if adjacent {
                        let number = number.parse::<usize>().expect("failed to parse digit");
                        println!("{r} {c} {number} adjacent");
                        solution += number;
                    } else {
                        println!("{r} {c} {number} not adjacent");
                    }

                    number = String::new();
                    adjacent = false;
                }

                continue;
            }

            // if we have a digit then we need to build an integer
            number.push(val.clone());

            if adjacent {
                continue;
            }

            // test to see if we have an adjacent symbol by looping around current index
            for t_r in -1..2 {
                for t_c in -1..2 {
                    let t_val = &data[(t_r + (r as isize)) as usize][(t_c + (c as isize)) as usize];

                    // avoid counting digits as symbols if they're part of the current number (on t_r 0)
                    let mut is_symbol = t_val != &period;
                    if t_r == 0 {
                        is_symbol &= !t_val.is_digit(RADIX);
                    }

                    adjacent |= is_symbol;

                    if adjacent {
                        println!("{t_r} {t_c} {t_val} {adjacent}");
                    }
                }

                if adjacent {
                    break;
                }
            }
        }
    }

    println!("Result: {}", solution);

    Ok(())
}
