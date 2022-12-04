use crate::common::parse_subsets;
use range_collections::{AbstractRangeSet, RangeSet};

pub fn count_intersections(lines: &Vec<String>) -> usize {
    let mut ranges = parse_subsets(lines);

    ranges
        .into_iter()
        .filter(|r| {
            let mut r = r.to_owned();
            r.0.intersection_with(&r.1);
            !r.0.is_empty()
        })
        .count()
}
