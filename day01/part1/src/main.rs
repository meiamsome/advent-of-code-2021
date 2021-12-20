use std::{fs::File, io::Read};

fn get_depth_increases(list: &Vec<u16>) -> usize {
    list.windows(2).filter(|res| res[0] < res[1]).count()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut contents = String::new();
    {
        let mut file = File::open("./day01/part1/input.txt")?;
        file.read_to_string(&mut contents)?;
    }

    let values = contents
        .split_whitespace()
        .map(|v| v.parse())
        .collect::<Result<_, _>>()?;

    let depth_increases = get_depth_increases(&values);

    println!("Total Depth Increases: {}", depth_increases);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(get_depth_increases(&input), 7);
    }
}
