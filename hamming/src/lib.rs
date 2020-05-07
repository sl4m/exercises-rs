pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if lens_match(s1, s2) {
        Some(calculate_diff(s1, s2))
    } else {
        None
    }
}

fn calculate_diff(s1: &str, s2: &str) -> usize {
    let mut s1_chars = s1.chars();
    let mut s2_chars = s2.chars();
    (0..s1.len()).fold(0, |mut acc, _| {
        let c1 = s1_chars.next().unwrap();
        let c2 = s2_chars.next().unwrap();
        if c1 != c2 {
            acc += 1
        }
        acc
    })
}

fn lens_match(s1: &str, s2: &str) -> bool {
    s1.len() == s2.len()
}
