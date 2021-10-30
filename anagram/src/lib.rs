use std::collections::{HashMap, HashSet};
type Counter = HashMap<char, i32>;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams: HashSet<&str> = HashSet::new();
    let word_counter = counter(word);
    for possible in possible_anagrams {
        if are_equal_no_case(word, possible) {
            continue;
        }
        if is_anagram(&word_counter, possible) {
            anagrams.insert(possible);
        }
    }
    anagrams
}

pub fn are_equal_no_case(word1: &str, word2: &str) -> bool {
    for (mut c1, mut c2) in word1.chars().zip(word2.chars()) {
        if c1.is_uppercase() {
            c1 = c1.to_lowercase().next().unwrap();
        }
        if c2.is_uppercase() {
            c2 = c2.to_lowercase().next().unwrap();
        }
        if c1 == c2 {
            continue;
        } else {
            return false;
        }
    }
    true
}

fn is_anagram<'a>(word_counter: &Counter, possible: &'a str) -> bool {
    return *word_counter == counter(possible);
}

fn counter(word: &str) -> Counter {
    let mut counter: Counter = HashMap::new();
    for c in word.chars() {
        let mut d = c;
        if c.is_uppercase() {
            d = c.to_lowercase().next().unwrap();
        }
        let v = counter.entry(d).or_insert(0);
        *v += 1;
    }
    counter
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    fn compare(w1: &str, w2: &str) {
        println!("w1 = {}, w2 = {}", w1, w2);
        let actual = w1.to_lowercase().to_string() == w2.to_lowercase().to_string();
        let expected = are_equal_no_case(w1, w2);
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_equality_1() {
        let w1 = "abc";
        let w2 = "abc";
        compare(w1, w2);
    }

    #[test]
    fn test_equality_2() {
        let w1 = "Abc";
        let w2 = "abc";
        compare(w1, w2);
    }

    #[test]
    fn test_equality_3() {
        let w1 = "Abc";
        let w2 = "aAbc";
        compare(w1, w2);
    }

    #[test]
    fn test_equality_4() {
        let w1 = "AbceFh";
        let w2 = "aBcEfh";
        compare(w1, w2);
    }
}
