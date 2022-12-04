use crate::common::parse_subsets;
use range_collections::AbstractRangeSet;

pub fn count_subsets(lines: &Vec<String>) -> usize {
    let ranges = parse_subsets(lines);

    ranges
        .into_iter()
        .filter(|r| r.0.is_subset(r.1.clone()) || r.1.is_subset(r.0.clone()))
        .count()
}
