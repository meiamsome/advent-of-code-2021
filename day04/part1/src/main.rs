use std::{fs::File, io::Read};

type Board = [[u8; 5]; 5];

fn parse_input<'a>(input: &'a str) -> (Vec<u8>, impl Iterator<Item = Board> + 'a) {
    let mut lines = input
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

    let called_numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(|number| number.parse().unwrap())
        .collect();

    let boards = std::iter::from_fn(move || {
        if let Some(_) = lines.peek() {
            let mut board: Board = [[0; 5]; 5];

            for row in 0..5 {
                let row_numbers: Vec<_> = lines
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|number| number.parse().unwrap())
                    .collect();
                for column in 0..5 {
                    board[row][column] = row_numbers[column];
                }
            }

            Some(board)
        } else {
            None
        }
    });

    (called_numbers, boards)
}

fn get_winning_combinations<'a>(board: &'a Board) -> impl Iterator<Item = [u8; 5]> + 'a {
    vec![(); 10].into_iter().enumerate().map(|(i, _)| {
        if i < 5 {
            board[i]
        } else {
            let column = i - 5;
            [
                board[0][column],
                board[1][column],
                board[2][column],
                board[3][column],
                board[4][column],
            ]
        }
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut contents = String::new();
    {
        let mut file = File::open("./day04/part1/input.txt")?;
        file.read_to_string(&mut contents)?;
    }

    let (numbers_called, boards) = parse_input(&contents);

    println!("Number Sequence: {:?}", numbers_called);
    println!("Board count: {}", boards.count());
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_parse() {
        let input = "
            7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

            22 13 17 11  0
             8  2 23  4 24
            21  9 14 16  7
             6 10  3 18  5
             1 12 20 15 19

             3 15  0  2 22
             9 18 13 17  5
            19  8  7 25 23
            20 11 10 24  4
            14 21 16 12  6

            14 21 17 24  4
            10 16 15  9 19
            18  8 23 26 20
            22 11 13  6  5
             2  0 12  3  7
        ";

        let (called_numbers, board_iterator) = parse_input(&input);

        assert_eq!(
            called_numbers,
            vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1
            ]
        );
        assert_eq!(
            board_iterator.collect::<Vec<_>>(),
            vec![
                [
                    [22, 13, 17, 11, 0],
                    [8, 2, 23, 4, 24],
                    [21, 9, 14, 16, 7],
                    [6, 10, 3, 18, 5],
                    [1, 12, 20, 15, 19],
                ],
                [
                    [3, 15, 0, 2, 22],
                    [9, 18, 13, 17, 5],
                    [19, 8, 7, 25, 23],
                    [20, 11, 10, 24, 4],
                    [14, 21, 16, 12, 6],
                ],
                [
                    [14, 21, 17, 24, 4],
                    [10, 16, 15, 9, 19],
                    [18, 8, 23, 26, 20],
                    [22, 11, 13, 6, 5],
                    [2, 0, 12, 3, 7],
                ],
            ],
        );
    }

    #[test]
    fn example_board_1_winning_combinations() {
        let board = [
            [22, 13, 17, 11, 0],
            [8, 2, 23, 4, 24],
            [21, 9, 14, 16, 7],
            [6, 10, 3, 18, 5],
            [1, 12, 20, 15, 19],
        ];

        let winning = get_winning_combinations(&board);

        assert_eq!(
            winning.collect::<Vec<_>>(),
            vec![
                [22, 13, 17, 11, 0],
                [8, 2, 23, 4, 24],
                [21, 9, 14, 16, 7],
                [6, 10, 3, 18, 5],
                [1, 12, 20, 15, 19],
                [22, 8, 21, 6, 1],
                [13, 2, 9, 10, 12],
                [17, 23, 14, 3, 20],
                [11, 4, 16, 18, 15],
                [0, 24, 7, 5, 19],
            ],
        );
    }
}
