use std::collections::HashMap;

pub fn convert(arabic: u32) -> String {
    let arabic_roman_tuples = [
        (10, String::from("X")),
        (9, String::from("IX")),
        (5, String::from("V")),
        (4, String::from("IV")),
        (1, String::from("I")),
    ];
    let arabic_to_roman_mapping: HashMap<u32, String> =
        arabic_roman_tuples.iter().cloned().collect();
    let arabics = arabic_roman_tuples
        .iter()
        .map(|t| t.0)
        .collect::<Vec<u32>>();
    let mut num = arabic;
    let mut result = String::new();

    for n in &arabics {
        while num >= *n {
            let roman = arabic_to_roman_mapping.get(&n);
            result.push_str(roman.unwrap());
            num = num - n;
        }
    }
    result
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn it_converts_1_to_I() {
        assert_eq!(convert(1), "I");
    }

    #[test]
    fn it_converts_2_to_II() {
        assert_eq!(convert(2), "II");
    }

    #[test]
    fn it_converts_4_to_IV() {
        assert_eq!(convert(4), "IV");
    }

    #[test]
    fn it_converts_5_to_V() {
        assert_eq!(convert(5), "V");
    }

    #[test]
    fn it_converts_6_to_VI() {
        assert_eq!(convert(6), "VI");
    }

    #[test]
    fn it_converts_9_to_IX() {
        assert_eq!(convert(9), "IX");
    }

    #[test]
    fn it_converts_10_to_X() {
        assert_eq!(convert(10), "X");
    }

    #[test]
    fn it_converts_20_to_XX() {
        assert_eq!(convert(20), "XX");
    }

    #[test]
    fn it_converts_26_to_XXVI() {
        assert_eq!(convert(26), "XXVI");
    }
}
