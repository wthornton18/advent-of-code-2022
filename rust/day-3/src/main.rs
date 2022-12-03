use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

mod common;
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
    println!("{}", part1::get_total_priority(&lines));
    println!("{}", part2::get_total_priority(&lines));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        let test_input = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw,",
        ]
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>();

        assert_eq!(part1::get_total_priority(&test_input), 157);
    }

    #[test]
    fn test_part2() {
        let test_input = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw,",
        ]
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>();

        assert_eq!(part2::get_total_priority(&test_input), 70);
    }
}
