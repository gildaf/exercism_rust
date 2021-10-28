use unicode_segmentation::UnicodeSegmentation;
pub fn reverse(input: &str) -> String {
    input.graphemes(true).rev().collect::<String>()
}

/// Process a single test case for the property `reverse`

#[cfg(test)]
mod tests {
    fn process_reverse_case(input: &str, expected: &str) {
        assert_eq!(&super::reverse(input), expected)
    }

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
        assert!(cfg!(feature = "grapheme"));
    }

    #[test]
    /// empty string
    fn test_an_empty_string() {
        process_reverse_case("", "");
    }

    #[test]
    /// a word
    fn test_a_word() {
        process_reverse_case("robot", "tobor");
    }

    #[test]
    /// a capitalized word
    fn test_a_capitalized_word() {
        process_reverse_case("Ramen", "nemaR");
    }

    #[test]
    /// a sentence with punctuation
    fn test_a_sentence_with_punctuation() {
        process_reverse_case("I'm hungry!", "!yrgnuh m'I");
    }

    #[test]
    /// a palindrome
    fn test_a_palindrome() {
        process_reverse_case("racecar", "racecar");
    }

    #[test]
    /// an even-sized word
    fn test_an_even_sized_word() {
        process_reverse_case("drawer", "reward");
    }

    #[test]
    /// wide characters
    fn test_wide_characters() {
        process_reverse_case("子猫", "猫子");
    }

    /// grapheme clusters
    #[test]
    #[cfg(feature = "grapheme")]
    fn test_grapheme_clusters() {
        process_reverse_case("uüu", "uüu");
    }
}
