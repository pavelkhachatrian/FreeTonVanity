use std::iter::Iterator;
use std::string::ToString;

pub struct BeautyAddressCheck {
    pub hex_letters: Vec<char>,
    pub hex_numbers: Vec<char>,
    pub hex_all_chars: Vec<char>,
    pub keywords: Vec<String>,
}

impl BeautyAddressCheck {
    const PL: usize = 7;
    pub fn new() -> Self {
        Self {
            hex_letters: "abcdef".chars().collect(),
            hex_numbers: "0123456789".chars().collect(),
            hex_all_chars: "0123456789abcdef".chars().collect(),
            keywords: vec![
                "abcabca".to_string(),
                "1234321".to_string(),
                "0123456".to_string(),
                "1234567".to_string(),
                "2345678".to_string(),
                "3456789".to_string(),
                "4567890".to_string(),
            ],
        }
    }

    #[allow(unused_doc_comments)]
    pub fn rule_beauty_address(&self, address: &str) -> u8 {
        let address: String = address.into();
        let mut char_vec: Vec<char> = address.chars().collect();

        /// check that in 8 chars only 2 or lower different unique chars
        let mut chunk: Vec<char> = vec![];
        for chunk_ in char_vec.chunks(8) {
            chunk = chunk_.into();
            chunk.sort();
            chunk.dedup();
            if chunk.len() < 3 {
                return 1;
            }
        }

        let mut prefix: Vec<char> = char_vec.clone()[0..BeautyAddressCheck::PL].to_vec();

        /// check for predefine keywords in prefix
        let prefix2: String = prefix.clone().into_iter().collect();
        if self.keywords.contains(&prefix2) {
            return 2;
        }

        /// check for 111111 122121 prefixes
        prefix.sort();
        prefix.dedup();
        if prefix.len() < 3 {
            return 3;
        }

        /// check that contains only 5 or lower different unique chars
        char_vec.sort();
        char_vec.dedup();
        if char_vec.len() < 6 {
            return 4;
        }

        /// check for only contains chars or only numbers
        let char_vec2 = char_vec.clone();
        if !char_vec.into_iter().map(|x| self.hex_numbers.contains(&x)).collect::<Vec<bool>>().contains(&false) ||
            !char_vec2.into_iter().map(|x| self.hex_letters.contains(&x)).collect::<Vec<bool>>().contains(&false) {
            return 5;
        }

        0
    }
}