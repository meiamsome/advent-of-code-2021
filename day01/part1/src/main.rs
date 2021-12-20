use std::{fs::File, io::Read};

fn get_depth_increases(iter: &mut dyn Iterator<Item = u16>) -> usize {
    let mut last_value = iter.next().unwrap();
    let mut increases = 0;
    for value in iter {
        if value > last_value {
            increases += 1;
        }
        last_value = value;
    }
    increases
}

fn main() -> std::io::Result<()> {
    let mut contents = String::new();
    {
        let mut file = File::open("./day01/part1/input.txt")?;
        file.read_to_string(&mut contents)?;
    }

    let depth_increases =
        get_depth_increases(&mut contents.split_whitespace().map(|v| v.parse().unwrap()));

    println!("Total Depth Increases: {}", depth_increases);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(get_depth_increases(&mut input.into_iter()), 7);
    }
}
