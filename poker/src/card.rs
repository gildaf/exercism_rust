// pub enum CardNumber {
//     Two,
//     Three,
//     Four,
//     Five,
//     Six,
//     Seven,
//     Eight,
//     Nine,
//     Ten,
//     Eleven
// }

// pub struct Card {
//     number: &i32,
//     symbol: &
// }
use std::str::from_utf8;
pub struct Parser<'a>(&'a str);

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Parser<'a> {
        Parser(input)
    }

    pub fn parts(&self) -> [&'a str; 5] {
        let a = self.0.split_ascii_whitespace().collect::<Vec<&str>>();
        a.try_into().unwrap()
    }

    pub fn symbols(&self) -> [char; 5] {
        self.parts()
            .iter()
            .map(|&s| s.chars().last().unwrap())
            .collect::<Vec<char>>()
            .try_into()
            .unwrap()
    }

    pub fn numbers(&self) -> [i32; 5] {
        let mut v = vec![];
        for &part in self.parts().iter() {
            let x = match part.len() {
                2 => {
                    let c = part.chars().next().unwrap();
                    if let Some(as_num) = c.to_digit(10) {
                        as_num as i32
                    } else {
                        match c {
                            'J' | 'j' => 11,
                            'Q' | 'q' => 12,
                            'K' | 'k' => 13,
                            'A' | 'a' => 14,
                            _ => panic!("bad input, {:?}", part),
                        }
                    }
                }
                3 => 10_i32,
                _ => panic!("wrong number of items in input string"),
            };
            v.push(x);
        }
        v.sort();
        v.reverse();
        v.try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parts() {
        // 4S 5S 7H 8D JC
        let s = "4S 5S 7H 8D JC";
        let _parts = Parser::new(s).parts();
        assert_eq!(_parts, ["4S", "5S", "7H", "8D", "JC"]);
    }

    #[test]
    fn test_symbols() {
        // 4S 5S 7H 8D JC
        let s = "4S 5S 7H 8D JC";
        let _parts = Parser::new(s).symbols();
        assert_eq!(_parts, ['S', 'S', 'H', 'D', 'C']);
    }

    #[test]
    fn test_numbers_1() {
        // 4S 5S 7H 8D JC
        let s = "4S AS KH 10D JC";
        let _parts = Parser::new(s).numbers();
        assert_eq!(_parts, [4, 14, 13, 10, 11]);
    }

    #[test]
    fn test_numbers_2() {
        // 4S 5S 7H 8D JC
        let s = "4S 5C 6H 7D 8C";
        let _parts = Parser::new(s).numbers();
        assert_eq!(_parts, [4, 5, 6, 7, 8]);
    }
}
