use std::collections::BinaryHeap;

pub fn find_max_calories_top_n(lines: &Vec<String>, n: usize) -> u64 {
    let mut heap: BinaryHeap<u64> = lines
        .split(|line| line.is_empty())
        .map(|group| group.iter().map(|v| v.parse::<u64>().unwrap()).sum())
        .collect();
    (0..n).map(|_| heap.pop().unwrap()).sum()
}
