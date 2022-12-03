use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

mod part1;
mod part2;

fn read_to_string<P>(p: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let mut buffer = String::new();
    let mut f = File::open(&p).unwrap();
    let _ = f.read_to_string(&mut buffer);

    buffer.lines().map(ToOwned::to_owned).collect::<Vec<_>>()
}

fn main() {
    let lines = read_to_string(PathBuf::from("./input.txt"));
    println!("{}", part1::find_max_calories(&lines));
    println!("{}", part2::find_max_calories_top_n(&lines, 3));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        let test_input = vec![
            "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
            "10000",
        ]
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>();

        assert_eq!(part1::find_max_calories(&test_input), 24000);
    }
    #[test]
    fn test_part2() {
        let test_input = vec![
            "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
            "10000",
        ]
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>();

        assert_eq!(part2::find_max_calories_top_n(&test_input, 3), 45000)
    }
}
