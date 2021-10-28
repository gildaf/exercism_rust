#![allow(unused)]
// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
// this is the solution to this exercise
// https://exercism.org/tracks/rust/exercises/low-power-embedded-game/
pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    return (dividend / divisor, dividend % divisor);
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    // unimplemented!("implement `fn evens`");
    // TODO: remove this; it's only necessary to allow this function to compile
    // before the student has done any work.
    // std::iter::empty()
    iter.enumerate().filter(|(i, _)| i % 2 == 0).map(|(i, v)| v)
}

use std::num;
pub struct Position(pub i16, pub i16);
impl Position {
    pub fn manhattan(&self) -> i16 {
        // let _abs = |num: i16| -> i16 {
        //     if num < 0 {
        //         -num
        //     } else {
        //         num
        //     }
        // };
        self.0.abs() + self.1.abs()
    }
    // fn abs(num: i16) -> i16 {
    //     match num {
    //         num if num < 0 => -num,
    //         _ => num,
    //     }
    // }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_divmod() {
        assert_eq!(super::divmod(13, 8), (1, 5));
        assert_eq!(super::divmod(22, 8), (2, 6));
    }

    #[test]
    fn test_evens_iter_from_evens() {
        let mut even_ints = super::evens(0_u8..);
        assert_eq!(even_ints.next(), Some(0));
        assert_eq!(even_ints.next(), Some(2));
        assert_eq!(even_ints.next(), Some(4));
        assert_eq!(even_ints.next(), Some(6));
    }

    #[test]
    fn test_evens_iter_from_nonevens() {
        let mut even_ints = super::evens(1_u8..);
        assert_eq!(even_ints.next(), Some(1));
        assert_eq!(even_ints.next(), Some(3));
        assert_eq!(even_ints.next(), Some(5));
        assert_eq!(even_ints.next(), Some(7));
    }
    #[test]
    fn test_manhattan() {
        use super::Position;
        assert_eq!(Position(3, 4).manhattan(), 7);
        assert_eq!(Position(3, -4).manhattan(), 7);
        assert_eq!(Position(-3, 4).manhattan(), 7);
        assert_eq!(Position(-3, -4).manhattan(), 7);
        assert_eq!(Position(12, 13).manhattan(), 25);
    }
}
