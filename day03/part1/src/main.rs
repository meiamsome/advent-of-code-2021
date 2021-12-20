use std::{fs::File, io::Read};

fn parse_input<'a>(input: &'a str) -> (usize, impl Iterator<Item = u16> + 'a) {
    let mut line_iter = input
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.len() > 0 {
                Some(line)
            } else {
                None
            }
        })
        .peekable();

    let bits = line_iter.peek().unwrap().len();

    (bits, line_iter.map(|x| u16::from_str_radix(x, 2).unwrap()))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut contents = String::new();
    {
        let mut file = File::open("./day01/part1/input.txt")?;
        file.read_to_string(&mut contents)?;
    }

    let _input = parse_input(&contents);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_parse() {
        let input = "
            00100
            11110
            10110
            10111
            10101
            01111
            00111
            11100
            10000
            11001
            00010
            01010
        ";

        let (bits, iter) = parse_input(&input);

        assert_eq!(bits, 5);
        assert_eq!(
            iter.collect::<Vec<_>>(),
            vec![4, 30, 22, 23, 21, 15, 7, 28, 16, 25, 2, 10],
        );
    }
}
