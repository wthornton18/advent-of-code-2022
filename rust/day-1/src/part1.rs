pub fn find_max_calories(lines: &Vec<String>) -> u64 {
    let max = lines
        .split(|line| line.is_empty())
        .map(|group| group.iter().map(|v| v.parse::<u64>().unwrap()).sum::<u64>())
        .max()
        .unwrap();
    max
}
