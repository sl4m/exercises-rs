const ISBN_LEN: usize = 10;

pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn = &isbn.replace("-", "")[..];
    is_valid_len(isbn) &&
        is_valid_format(isbn) &&
        is_valid(isbn)
}

fn is_valid_len(isbn: &str) -> bool {
    isbn.len() == ISBN_LEN
}

fn is_valid_format(isbn: &str) -> bool {
    let mut isbn_chars = isbn.chars();
    (0..9).all(|_| is_dec_digit(isbn_chars.next().unwrap())) &&
        get_check_char(isbn_chars.next().unwrap()).is_some()
}

fn is_valid(isbn: &str) -> bool {
    let mut isbn_chars = isbn.chars();
    let partial_sum = (2..11).rev().fold(0, |acc, x| {
        let isbn_digit = to_dec_digit(isbn_chars.next().unwrap());
        acc + (isbn_digit * x)
    });
    if let Some(check_char_value) = get_check_char(isbn_chars.next().unwrap()) {
        (partial_sum + check_char_value) % 11 == 0
    } else {
        false
    }
}

fn get_check_char(check_char: char) -> Option<u32> {
    match check_char {
        'X' => Some(10),
        x if is_dec_digit(x) => Some(to_dec_digit(x)),
        _ => None,
    }
}

fn is_dec_digit(c: char) -> bool {
    c.is_digit(10)
}

fn to_dec_digit(c: char) -> u32 {
    c.to_digit(10).unwrap()
}
