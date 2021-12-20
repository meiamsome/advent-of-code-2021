use std::{fs::File, io::Read};

type Coordinate = u32;

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    Forward(Coordinate),
    Down(Coordinate),
    Up(Coordinate),
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
        .map(|line| {
            let (direction, len) = line.split_once(' ').unwrap();
            let len = len.parse().unwrap();
            match direction {
                "forward" => Instruction::Forward(len),
                "down" => Instruction::Down(len),
                "up" => Instruction::Up(len),
                _ => panic!(),
            }
        })
}

fn calculate_position(
    instructions: &mut dyn Iterator<Item = Instruction>,
) -> (Coordinate, Coordinate, Coordinate) {
    instructions.fold(
        (0, 0, 0),
        |(aim, horizontal, depth), instruction| match instruction {
            Instruction::Forward(distance) => (aim, horizontal + distance, depth + distance * aim),
            Instruction::Down(aim_delta) => (aim + aim_delta, horizontal, depth),
            Instruction::Up(aim_delta) => (aim - aim_delta, horizontal, depth),
        },
    )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut contents = String::new();
    {
        let mut file = File::open("./day02/part1/input.txt")?;
        file.read_to_string(&mut contents)?;
    }

    let mut instructions = parse_input(&contents);

    let (_aim, horizontal, depth) = calculate_position(&mut instructions);

    println!("Coords: {}, {}", horizontal, depth);

    println!("Solution: {}", horizontal * depth);

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

    #[test]
    fn example_position() {
        let input = vec![
            Instruction::Forward(5),
            Instruction::Down(5),
            Instruction::Forward(8),
            Instruction::Up(3),
            Instruction::Down(8),
            Instruction::Forward(2),
        ];

        let (_aim, horizontal, depth) = calculate_position(&mut input.into_iter());

        assert_eq!(horizontal, 15);
        assert_eq!(depth, 60);
    }
}
