use lazy_regex::{regex, Captures, Lazy, Regex};
use range_collections::RangeSet;

static R: &Lazy<Regex> = regex!(r"(\d+)-(\d+),(\d+)-(\d+)");

pub fn parse_subsets(lines: &Vec<String>) -> Vec<(RangeSet<[usize; 2]>, RangeSet<[usize; 2]>)> {
    let mut ranges = Vec::new();
    for line in lines {
        let captures = R.captures(line);
        if let Some(captures) = captures {
            let first_range_start = parse_capture(&captures, 1);
            let first_range_end = parse_capture(&captures, 2);
            let second_range_start = parse_capture(&captures, 3);
            let second_range_end = parse_capture(&captures, 4);
            let first_range: RangeSet<[usize; 2]> =
                RangeSet::from(first_range_start..first_range_end + 1);
            let second_range: RangeSet<[usize; 2]> =
                RangeSet::from(second_range_start..second_range_end + 1);
            ranges.push((first_range, second_range))
        }
    }
    ranges
}

fn parse_capture(capture: &Captures, i: usize) -> usize {
    capture
        .get(i)
        .map(|s| s.as_str())
        .map(|s| s.parse::<usize>().unwrap())
        .unwrap()
}
