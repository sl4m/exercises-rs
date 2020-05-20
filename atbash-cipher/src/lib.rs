static ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyz";
static OBFUSCATION_COUNT: usize = 5;

pub fn encode(plain: &str) -> String {
    obfuscate(&transform(plain))
}

pub fn decode(cipher: &str) -> String {
    transform(cipher)
}

fn transform(input: &str) -> String {
    input.chars()
        .filter_map(|c| {
            if c.is_digit(10) {
                Some(c)
            } else if let Some(i) = ALPHABET.find(c.to_ascii_lowercase()) {
                ALPHABET.chars().nth_back(i)
            } else {
                None
            }})
        .collect::<String>()
}

fn obfuscate(encoded: &str) -> String {
    encoded.chars()
        .enumerate()
        .fold("".to_string(), |mut acc, (i, c)| {
            if i != 0 && (i % OBFUSCATION_COUNT) == 0 {
                acc.push(' ');
            }
            acc.push(c);
            acc})
}
