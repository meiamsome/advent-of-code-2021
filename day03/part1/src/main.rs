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

fn calculate_rates(bits: usize, diagnostic_input: &mut dyn Iterator<Item = u16>) -> (u16, u16) {
    let mut counters = vec![0usize; bits];
    let mut total = 0;
    for value in diagnostic_input {
        total += 1;
        for i in 0..bits {
            if value & (1 << i) != 0 {
                counters[i] += 1;
            }
        }
    }

    let mut epsilon = 0;
    let mut gamma = 0;
    for i in 0..bits {
        if counters[i] > total / 2 {
            gamma |= 1 << i;
        } else {
            epsilon |= 1 << i;
        }
    }

    (gamma, epsilon)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut contents = String::new();
    {
        let mut file = File::open("./day03/part1/input.txt")?;
        file.read_to_string(&mut contents)?;
    }

    let (bits, mut diagnostic_input) = parse_input(&contents);
    let (gamma, epsilon) = calculate_rates(bits, &mut diagnostic_input);

    println!("Power consumption {}", gamma as u32 * epsilon as u32);

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

    #[test]
    fn example_calculate() {
        let input = vec![4, 30, 22, 23, 21, 15, 7, 28, 16, 25, 2, 10];

        let (gamma, epsilon) = calculate_rates(5, &mut input.into_iter());

        assert_eq!(gamma, 22);
        assert_eq!(epsilon, 9);
    }
}
