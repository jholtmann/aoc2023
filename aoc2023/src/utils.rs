use std::collections::HashSet;

pub const RADIX: u32 = 10;

/// Read a single integer from a string.
pub fn read_int(line: &str) -> anyhow::Result<(usize, usize)> {
    println!("{}", line);
    let mut chars = String::from("");

    let mut index = 0;
    for (i, c) in line.chars().enumerate() {
        if !c.is_digit(RADIX) {
            index = i;
            break;
        }

        chars.push(c);
    }

    Ok((chars.parse::<usize>()?, index))
}

/// Parse delimited integers from a string.
pub fn read_ints(line: &str, delimiter: char) -> HashSet<usize> {
    line.split(delimiter)
        .filter(|&x| !x.is_empty())
        .map(|x| x.trim().parse::<usize>().expect("failed to parse to number"))
        .collect::<HashSet<usize>>()
}