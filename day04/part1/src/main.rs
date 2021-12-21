use std::{fs::File, io::Read};

use day04_part1::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut contents = String::new();
    {
        let mut file = File::open("./day04/part1/input.txt")?;
        file.read_to_string(&mut contents)?;
    }

    let (numbers_called, mut boards) = parse_input(&contents);

    let numbers_called: Vec<_> = numbers_called.collect();

    let numbers_called_map = called_numbers_to_map(&mut numbers_called.clone().into_iter());
    let (winning_board, winning_index) = get_winning_board(&numbers_called_map, &mut boards);

    let winning_number = numbers_called[winning_index];

    let winning_board_score = get_board_score(
        &numbers_called_map,
        &winning_board,
        winning_index,
        winning_number,
    );

    println!("Winning Board: {:?}", winning_board);
    println!("Winning Board Score: {}", winning_board_score);

    Ok(())
}
