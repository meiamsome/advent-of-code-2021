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

fn calculate_oxygen_generator(bits: usize, diagnostic_input: Vec<u16>) -> u16 {
    let mut data = diagnostic_input;

    for i in (0..bits).rev() {
        if data.len() == 1 {
            return data[0];
        }
        let bit_selection = 1 << i;

        let bits = data.iter().filter(|&x| x & bit_selection != 0).count();
        let len = data.len();

        let bit_mask = if bits * 2 >= len { bit_selection } else { 0 };
        data = data
            .into_iter()
            .filter(|&value| (value ^ bit_mask) & bit_selection == 0)
            .collect();
    }

    data[0]
}

fn calculate_co2_scrubber(bits: usize, diagnostic_input: Vec<u16>) -> u16 {
    let mut data = diagnostic_input;

    for i in (0..bits).rev() {
        if data.len() == 1 {
            return data[0];
        }
        let bit_selection = 1 << i;

        let bits = data.iter().filter(|&x| x & bit_selection != 0).count();
        let len = data.len();

        let bit_mask = if bits * 2 < len { bit_selection } else { 0 };
        data = data
            .into_iter()
            .filter(|&value| (value ^ bit_mask) & bit_selection == 0)
            .collect();
    }

    data[0]
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut contents = String::new();
    {
        let mut file = File::open("./day03/part1/input.txt")?;
        file.read_to_string(&mut contents)?;
    }

    let (bits, diagnostic_input) = parse_input(&contents);

    let diagnostic_input: Vec<_> = diagnostic_input.collect();

    let oxygen_generator = calculate_oxygen_generator(bits, diagnostic_input.clone());
    println!("Oxygen Generator: {}", oxygen_generator);

    let co2_scrubber = calculate_co2_scrubber(bits, diagnostic_input);
    println!("CO2 Scrubber: {}", co2_scrubber);

    println!(
        "Solution: {}",
        oxygen_generator as u32 * co2_scrubber as u32
    );

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
    fn example_calculate_oxygen_generator() {
        let input = vec![4, 30, 22, 23, 21, 15, 7, 28, 16, 25, 2, 10];

        let oxygen_generator = calculate_oxygen_generator(5, input);

        assert_eq!(oxygen_generator, 23);
    }

    #[test]
    fn example_calculate_co2_scrubber() {
        let input = vec![4, 30, 22, 23, 21, 15, 7, 28, 16, 25, 2, 10];

        let co2_scrubber = calculate_co2_scrubber(5, input);

        assert_eq!(co2_scrubber, 10);
    }
}
