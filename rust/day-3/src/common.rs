pub fn priority(c: &char) -> Option<u32> {
    match c {
        'a'..='z' => Some((c.to_owned() as u32) - 96),
        'A'..='Z' => Some((c.to_owned() as u32) - 38),
        _ => None,
    }
}
