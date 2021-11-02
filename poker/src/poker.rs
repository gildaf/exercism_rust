pub mod poker {
    type RawHand<'a> = &'a str;
    pub enum PokerHand<'a> {
        HighCard(HighCardHand<'a>),
        OnePair(RawHand<'a>),
    }

    pub struct HighCardHand<'a> {
        inner: &'a str,
    }

    impl<'a> HighCardHand<'a> {
        pub fn new(hand: RawHand) -> PokerHand {
            PokerHand::HighCard(HighCardHand { inner: hand })
        }

        // pub fn high_card() -> i32 {
        // }
    }

    impl<'a> PokerHand<'a> {
        pub fn new(hand: RawHand) -> PokerHand {}
    }
    pub mod logic {
        // fn is_one_pair('')
    }
}
