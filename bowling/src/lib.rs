#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, PartialEq)]
pub enum Roll {
    Strike,
    Spare(u16),
    Numeric(u16),
}

impl Roll {
    fn new_first(pins: u16) -> Result<Roll, Error> {
        if pins > 10 {
            Err(Error::NotEnoughPinsLeft)
        } else if pins == 10 {
            Ok(Self::Strike)
        } else {
            Ok(Self::Numeric(pins))
        }
    }
    fn new_second(pins: u16, prev: u16) -> Result<Roll, Error> {
        if pins + prev > 10 {
            Err(Error::NotEnoughPinsLeft)
        } else if pins + prev == 10 {
            Ok(Self::Spare(pins))
        } else {
            Ok(Self::Numeric(pins))
        }
    }
    fn value(&self) -> u16 {
        match self {
            Self::Strike => 10,
            Self::Spare(v) => *v,
            Self::Numeric(v) => *v,
        }
    }
}
pub struct BowlingGame {
    frames: Vec<Frame>,
}

#[derive(Debug)]
struct RegularFrame {
    rolls: [Option<Roll>; 2],
}
#[derive(Debug)]
struct FinalFrame {
    rolls: [Option<Roll>; 3],
}

#[derive(Debug)]
enum Frame {
    Regular(RegularFrame),
    Final(FinalFrame),
}

impl Frame {
    fn new(index: usize) -> Frame {
        if index == 10 {
            Frame::Final(FinalFrame::new())
        } else {
            Frame::Regular(RegularFrame::new())
        }
    }
    fn total(&self) -> u16 {
        match self {
            Frame::Final(FinalFrame { rolls, .. }) => rolls
                .iter()
                .map(|x| if let Some(v) = x { v.value() } else { 0 })
                .sum(),
            Frame::Regular(RegularFrame { rolls, .. }) => rolls
                .iter()
                .map(|x| if let Some(v) = x { v.value() } else { 0 })
                .sum(),
        }
    }
    fn is_done(&self) -> bool {
        match self {
            Self::Final(frame) => frame.is_done(),
            Self::Regular(frame) => frame.is_done(),
        }
    }
    fn roll(&mut self, pins: u16) -> Result<(), Error> {
        match self {
            Self::Final(frame) => frame.roll(pins),
            Self::Regular(frame) => frame.roll(pins),
        }
    }
    fn roll1(&self) -> u16 {
        let roll = match self {
            Self::Final(frame) => frame.rolls[0].as_ref(),
            Self::Regular(frame) => frame.rolls[0].as_ref(),
        };
        roll.as_ref().unwrap().value()
    }
    fn roll2(&self) -> u16 {
        let roll = match self {
            Self::Final(frame) => frame.rolls[1].as_ref(),
            Self::Regular(frame) => frame.rolls[1].as_ref(),
        };
        roll.as_ref().unwrap().value()
    }
}

impl FinalFrame {
    fn new() -> FinalFrame {
        FinalFrame {
            rolls: [None, None, None],
        }
    }

    fn is_done(&self) -> bool {
        match self.rolls {
            [None, None, None] => false,
            [Some(_), None, None] => false,
            [Some(Roll::Strike), _, None] => false,
            [Some(_), Some(Roll::Strike), None] => false,
            [Some(_), Some(Roll::Spare(_)), None] => false,
            _ => true,
        }
    }
    fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }
        match self.rolls {
            [None, None, None] => {
                //1
                self.rolls[0] = Some(Roll::new_first(pins)?);
                Ok(())
            }
            [Some(Roll::Numeric(v1)), None, None] => {
                // 2
                self.rolls[1] = Some(Roll::new_second(pins, v1)?);
                Ok(())
            }
            [Some(Roll::Strike), None, None] => {
                // 2
                self.rolls[1] = Some(Roll::new_first(pins)?);
                Ok(())
            }
            [Some(Roll::Numeric(_v1)), Some(Roll::Numeric(_v2)), None] => {
                //3 -- if no throw is spare or stike then we are only getting
                // two rolls.
                Err(Error::NotEnoughPinsLeft)
            }
            [Some(Roll::Numeric(_v1)), Some(Roll::Spare(_v2)), None] => {
                //3
                self.rolls[2] = Some(Roll::new_first(pins)?);
                Ok(())
            }
            [Some(Roll::Strike), Some(Roll::Numeric(v1)), None] => {
                self.rolls[2] = Some(Roll::new_second(pins, v1)?);
                Ok(())
            }
            [Some(Roll::Strike), Some(Roll::Strike), None] => {
                self.rolls[2] = Some(Roll::new_first(pins)?);
                Ok(())
            }
            _ => Err(Error::NotEnoughPinsLeft),
        }
    }
}

impl RegularFrame {
    fn new() -> RegularFrame {
        RegularFrame {
            rolls: [None, None],
        }
    }
    fn is_done(&self) -> bool {
        match self.rolls {
            [None, None] => false,
            [Some(Roll::Strike), None] => true,
            [Some(Roll::Numeric(_)), None] => false,
            [Some(_), Some(_)] => true,
            _ => panic!("i dont know: {:?}", &self),
        }
    }
    fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }
        match self.rolls {
            [None, None] => {
                self.rolls[0] = Some(Roll::new_first(pins)?);
                Ok(())
            }
            [Some(Roll::Strike), None] => Err(Error::NotEnoughPinsLeft),
            [Some(Roll::Numeric(v1)), None] => {
                self.rolls[1] = Some(Roll::new_second(pins, v1)?);
                Ok(())
            }
            _ => Err(Error::NotEnoughPinsLeft),
        }
    }
    fn is_strike(&self) -> bool {
        match self.rolls[0] {
            Some(Roll::Strike) => true,
            _ => false,
        }
    }
    fn is_spare(&self) -> bool {
        match self.rolls[1] {
            Some(Roll::Spare(_)) => true,
            _ => false,
        }
    }
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            frames: vec![Frame::new(1)],
        }
    }
    fn mut_last_frame(&mut self) -> &mut Frame {
        return self.frames.last_mut().unwrap();
    }
    fn is_game_done(&self) -> bool {
        self.frames.len() == 10 && self.is_last_frame_done()
    }
    fn is_last_frame_done(&self) -> bool {
        self.frames.last().unwrap().is_done()
    }
    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.is_game_done() {
            return Err(Error::GameComplete);
        }
        if self.is_last_frame_done() {
            let new_frame = Frame::new(self.frames.len() + 1);
            self.frames.push(new_frame)
        }
        self.mut_last_frame().roll(pins)
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_game_done() {
            return None;
        }
        let mut scores = vec![0_u16; 10];
        for (i, frame) in self.frames.iter().enumerate() {
            let local = frame.total();
            scores[i] = match frame {
                Frame::Regular(reg) => {
                    if reg.is_strike() {
                        let next_frame = &self.frames[i + 1];
                        match next_frame {
                            Frame::Regular(reg) if reg.is_strike() => {
                                let next_next_frame = &self.frames[i + 2];
                                local + next_frame.roll1() + next_next_frame.roll1()
                            }
                            _ => local + next_frame.roll1() + next_frame.roll2(),
                        }
                    } else if reg.is_spare() {
                        let next_frame = &self.frames[i + 1];
                        local + next_frame.roll1()
                    } else {
                        local
                    }
                }
                Frame::Final(_) => local,
            }
        }
        return Some(scores.iter().sum());
    }
}

#[cfg(test)]
mod tests {
    use super::{BowlingGame, Frame};
    #[test]
    fn test_starting_at_0() {
        let g = BowlingGame::new();
        assert_eq!(g.score(), None);
    }

    #[test]
    fn create_new_regular_frame() {
        let f = Frame::new(1);
        match f {
            Frame::Regular(_) => (),
            Frame::Final(_) => assert!(false),
        }
        assert_eq!(f.is_done(), false);
    }
    #[test]
    fn create_new_final_frame() {
        let f = Frame::new(10);
        match f {
            Frame::Regular(_) => assert!(false),
            Frame::Final(_) => (),
        }
        assert_eq!(f.is_done(), false);
    }
    #[test]
    fn test_frame_two_rolls() {
        let mut f = Frame::new(1);
        assert_eq!(f.is_done(), false);
        assert_eq!(f.roll(1), Ok(()));
        assert_eq!(f.is_done(), false);
        assert_eq!(f.roll(8), Ok(()));
        assert_eq!(f.is_done(), true);
        assert_eq!(f.total(), 9);
    }

    #[test]
    fn test_frame_strike() {
        let mut f = Frame::new(1);
        assert_eq!(f.is_done(), false);
        assert_eq!(f.roll(10), Ok(()));
        assert_eq!(f.is_done(), true);
        assert_eq!(f.total(), 10);
    }

    #[test]
    fn test_frame_spare() {
        let mut f = Frame::new(1);
        assert_eq!(f.is_done(), false);
        assert_eq!(f.roll(1), Ok(()));
        assert_eq!(f.is_done(), false);
        assert_eq!(f.roll(9), Ok(()));
        assert_eq!(f.is_done(), true);
        assert_eq!(f.total(), 10);
    }

    #[test]
    fn test_frame_spare_on_last_frame() {
        let mut f = Frame::new(10);
        assert_eq!(f.is_done(), false);

        assert_eq!(f.roll(1), Ok(()));
        assert_eq!(f.is_done(), false);
        assert_eq!(f.total(), 1);

        assert_eq!(f.roll(9), Ok(()));
        assert_eq!(f.is_done(), false);
        assert_eq!(f.total(), 10);

        assert_eq!(f.roll(7), Ok(()));
        assert_eq!(f.is_done(), true);
        assert_eq!(f.total(), 17);
    }

    #[test]
    fn test_last_frame_strike_and_spare() {
        let mut f = Frame::new(10);
        assert_eq!(f.is_done(), false);

        assert_eq!(f.roll(10), Ok(()));
        assert_eq!(f.is_done(), false);
        assert_eq!(f.total(), 10);

        assert_eq!(f.roll(7), Ok(()));
        assert_eq!(f.is_done(), false);
        assert_eq!(f.total(), 17);

        assert_eq!(f.roll(3), Ok(()));
        assert_eq!(f.is_done(), true);
        assert_eq!(f.total(), 20);
    }

    #[test]
    fn test_two_rolls_last_frame() {
        let mut f = Frame::new(10);
        assert_eq!(f.is_done(), false);
        assert_eq!(f.roll(1), Ok(()));
        assert_eq!(f.is_done(), false);
        assert_eq!(f.roll(8), Ok(()));
        assert_eq!(f.is_done(), true);
        assert_eq!(f.total(), 9);
    }

    #[test]
    fn test_three_strikes_last_frame() {
        let mut f = Frame::new(10);
        assert_eq!(f.is_done(), false);
        assert_eq!(f.roll(10), Ok(()));
        assert_eq!(f.is_done(), false);
        assert_eq!(f.roll(10), Ok(()));
        assert_eq!(f.is_done(), false);
        assert_eq!(f.roll(10), Ok(()));
        assert_eq!(f.is_done(), true);
        assert_eq!(f.total(), 30);
    }

    #[test]
    fn test_simple_game_1() {
        let mut g = BowlingGame::new();
        assert_eq!(g.score(), None);
        for _r in 1..21 {
            g.roll(1).unwrap();
        }
        assert_eq!(g.is_game_done(), true);
        assert_eq!(g.frames.len(), 10);

        assert_eq!(g.score(), Some(20));
    }

    #[test]
    fn test_prefect_game() {
        let mut g = BowlingGame::new();
        assert_eq!(g.score(), None);
        for _r in 0..12 {
            g.roll(10).unwrap();
        }
        assert_eq!(g.is_game_done(), true);
        assert_eq!(g.frames.len(), 10);

        assert_eq!(g.score(), Some(300));
    }

    #[test]
    fn a_spare_with_the_two_roll_bonus_does_not_get_a_bonus_roll() {
        let mut game = BowlingGame::new();
        for _ in 0..18 {
            let _ = game.roll(0);
        }
        let _ = game.roll(10).unwrap();
        let _ = game.roll(7).unwrap();
        let _ = game.roll(3).unwrap();
        assert_eq!(game.score(), Some(20));
    }
}
