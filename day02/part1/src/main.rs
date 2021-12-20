use std::{fs::File, io::Read, iter::Inspect};

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    Forward(u16),
    Down(u16),
    Up(u16),
}

fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = Instruction> + 'a {
    input
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.len() > 0 {
                Some(line)
            } else {
                None
            }
        })
        .map(|x| {
            let (direction, len) = x.trim().split_once(' ').unwrap();
            let len = len.parse().unwrap();
            match direction {
                "forward" => Instruction::Forward(len),
                "down" => Instruction::Down(len),
                "up" => Instruction::Up(len),
                _ => panic!(),
            }
        })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut contents = String::new();
    {
        let mut file = File::open("./day02/part1/input.txt")?;
        file.read_to_string(&mut contents)?;
    }

    println!("TODO");

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_parse() {
        let input = "
            forward 5
            down 5
            forward 8
            up 3
            down 8
            forward 2
        ";

        assert_eq!(
            parse_input(&input).collect::<Vec<_>>(),
            vec![
                Instruction::Forward(5),
                Instruction::Down(5),
                Instruction::Forward(8),
                Instruction::Up(3),
                Instruction::Down(8),
                Instruction::Forward(2),
            ]
        );
    }
}
