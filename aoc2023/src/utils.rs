pub const RADIX: u32 = 10;

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
