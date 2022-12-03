use crate::common::priority;
use std::collections::HashSet;

pub fn get_total_priority(lines: &Vec<String>) -> usize {
    let out = lines
        .chunks(3)
        .map(|chunks| {
            let sets = chunks
                .iter()
                .map(|chars| chars.chars().collect::<HashSet<_>>())
                .collect::<Vec<HashSet<_>>>();

            let intersection = sets
                .iter()
                .skip(1)
                .fold(sets[0].clone(), |acc, hs| {
                    acc.intersection(hs).cloned().collect()
                })
                .iter()
                .filter_map(|c| priority(c))
                .map(|c| c as usize)
                .sum::<usize>();
            intersection
        })
        .sum::<usize>();
    out
}
