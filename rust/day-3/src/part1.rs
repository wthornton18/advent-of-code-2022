use crate::common::priority;
use std::collections::HashSet;

pub fn get_total_priority(lines: &Vec<String>) -> usize {
    lines
        .iter()
        .map(|line| {
            let (first_chars, second_chars) = line.split_at(line.len() / 2);
            let first_items = first_chars.chars().collect::<HashSet<_>>();
            let second_items: HashSet<char> = second_chars.chars().collect::<HashSet<_>>();
            let priority = first_items
                .intersection(&second_items)
                .filter_map(|c| priority(c))
                .map(|v| v as usize)
                .sum::<usize>();
            priority
        })
        .sum()
}
