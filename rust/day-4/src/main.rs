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
    println!("{}", part1::count_subsets(&lines));
    println!("{}", part2::count_intersections(&lines));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        let lines = vec![
            "2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8",
        ]
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>();

        assert_eq!(part1::count_subsets(&lines), 2)
    }

    #[test]
    fn test_part2() {
        let lines = vec![
            "2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8",
        ]
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>();

        assert_eq!(part2::count_intersections(&lines), 4);
    }
}
