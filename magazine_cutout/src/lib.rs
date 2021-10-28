use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_words = counter(magazine);
    for &word in note {
        match magazine_words.get_mut(word) {
            Some(v) if *v > 0 => *v -= 1,
            _ => return false,
        }
    }
    true
}

fn counter<'a>(magazine: &[&'a str]) -> HashMap<&'a str, i32> {
    let mut mymap: HashMap<&str, i32> = HashMap::new();
    for &word in magazine {
        mymap.entry(word).and_modify(|v| *v += 1).or_insert(1);
    }
    mymap
}

// pub fn g_counter<'a>(magazine: Iterator<&'a str>) -> HashMap<&'a str, i32> {
//     let mut mymap: HashMap<&str, i32> = HashMap::new();
//     for &word in magazine {
//         let value = mymap.entry(word).or_insert(0);
//         *value += 1;
//     }
//     mymap
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    use super::{can_construct_note, counter};

    #[test]
    fn test_counter_from_words() {
        let s = "this is me is me".split_whitespace().collect::<Vec<&str>>();
        let mut c = counter(&s);
        assert_eq!(c.get(&"aaa"), None);
        assert_eq!(c.get(&"this"), Some(&1));
        assert_eq!(c.get(&"is"), Some(&2));
        assert_eq!(c.get(&"me"), Some(&2));
        c.entry(&"me").and_modify(|v| *v -= 2);
        assert_eq!(c.get(&"me"), Some(&0));
    }

    #[test]
    fn test_can_construct() {
        let magazine = "two times three is not four"
            .split_whitespace()
            .collect::<Vec<&str>>();
        let note = "two times two is four"
            .split_whitespace()
            .collect::<Vec<&str>>();
        let m = &magazine;
        assert!(!can_construct_note(m, &note));
    }
}
