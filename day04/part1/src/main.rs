use std::{collections::HashMap, fs::File, io::Read};

type Board = [[u8; 5]; 5];

fn parse_input<'a>(
    input: &'a str,
) -> (
    impl Iterator<Item = u8> + 'a,
    impl Iterator<Item = Board> + 'a,
) {
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
        .map(|number| number.parse().unwrap());

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

fn called_numbers_to_map(called_numbers: &mut dyn Iterator<Item = u8>) -> HashMap<u8, usize> {
    called_numbers
        .enumerate()
        .map(|(index, number)| (number, index))
        .collect()
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

fn get_win_time(numbers_called: &HashMap<u8, usize>, board: &Board) -> usize {
    get_winning_combinations(board)
        .map(|combo| {
            combo
                .into_iter()
                .map(|n| *numbers_called.get(&n).ok_or(usize::MAX).unwrap())
                .max()
                .unwrap()
        })
        .min()
        .unwrap()
}

fn get_winning_board(
    numbers_called: &HashMap<u8, usize>,
    boards: &mut dyn Iterator<Item = Board>,
) -> Board {
    boards.min_by_key(|board| get_win_time(numbers_called, board)).unwrap()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut contents = String::new();
    {
        let mut file = File::open("./day04/part1/input.txt")?;
        file.read_to_string(&mut contents)?;
    }

    let (mut numbers_called, mut boards) = parse_input(&contents);

    let numbers_called_map = called_numbers_to_map(&mut numbers_called);
    let winning_board = get_winning_board(&numbers_called_map, &mut boards);

    println!("Winning Board: {:?}", winning_board);

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
            called_numbers.collect::<Vec<_>>(),
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
    fn example_called_numbers_to_map() {
        let numbers_called = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];

        let result = called_numbers_to_map(&mut numbers_called.into_iter());

        assert_eq!(
            result,
            vec![
                (7, 0),
                (4, 1),
                (9, 2),
                (5, 3),
                (11, 4),
                (17, 5),
                (23, 6),
                (2, 7),
                (0, 8),
                (14, 9),
                (21, 10),
                (24, 11),
                (10, 12),
                (16, 13),
                (13, 14),
                (6, 15),
                (15, 16),
                (25, 17),
                (12, 18),
                (22, 19),
                (18, 20),
                (20, 21),
                (8, 22),
                (19, 23),
                (3, 24),
                (26, 25),
                (1, 26),
            ]
            .into_iter()
            .collect(),
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

    #[test]
    fn example_board_1_win_time() {
        let numbers_called = called_numbers_to_map(
            &mut vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1,
            ]
            .into_iter(),
        );

        let board = [
            [22, 13, 17, 11, 0],
            [8, 2, 23, 4, 24],
            [21, 9, 14, 16, 7],
            [6, 10, 3, 18, 5],
            [1, 12, 20, 15, 19],
        ];

        let time = get_win_time(&numbers_called, &board);

        assert_eq!(time, 13);
    }

    #[test]
    fn example_board_3_win_time() {
        let numbers_called = called_numbers_to_map(
            &mut vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1,
            ]
            .into_iter(),
        );

        let board = [
            [14, 21, 17, 24, 4],
            [10, 16, 15, 9, 19],
            [18, 8, 23, 26, 20],
            [22, 11, 13, 6, 5],
            [2, 0, 12, 3, 7],
        ];

        let time = get_win_time(&numbers_called, &board);

        assert_eq!(time, 11);
    }

    #[test]
    fn example_get_winning_board() {
        let numbers_called = called_numbers_to_map(
            &mut vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1,
            ]
            .into_iter(),
        );

        let boards = vec![
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
        ];

        let winning_board = get_winning_board(&numbers_called, &mut boards.clone().into_iter());

        assert_eq!(winning_board, boards[2]);
    }
}
