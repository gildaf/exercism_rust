// use chrono::{DateTime, Utc};
use std::fmt;

#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            hours: (hours % 24).abs(),
            minutes: (minutes % 60).abs(),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut hours = ((minutes / 60) + self.hours) % 24;
        let mut minutes = (minutes % 60) + self.minutes;
        if hours < 0 {
            hours += 24;
        }
        if minutes > 60 {
            minutes %= 60;
            hours += 1;
            if hours > 23 {
                hours -= 24;
            }
        }
        if minutes < 0 {
            hours -= 1;
            if hours == -1 {
                hours = 23;
            }
            minutes += 60;
        }
        return Clock {
            hours: hours,
            minutes: minutes,
        };
    }

    pub fn as_tuple(&self) -> (i32, i32) {
        self.into()
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl From<Clock> for (i32, i32) {
    fn from(c: Clock) -> (i32, i32) {
        let Clock { hours, minutes } = c;
        (hours, minutes)
    }
}

impl From<&Clock> for (i32, i32) {
    fn from(c: &Clock) -> (i32, i32) {
        let Clock { hours, minutes } = c;
        (*hours, *minutes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_new_clock() {
        let clock = Clock::new(1, 2);
        // check_clock(c, (1,1));
        assert_eq!(clock.hours, 1);
        assert_eq!(clock.minutes, 2);
    }

    #[test]
    fn test_traits_sanity() {
        assert_eq!(Clock::new(15, 37), Clock::new(15, 37))
    }
    #[test]
    fn test_into_tuple() {
        let clock = Clock::new(10, 20);
        assert_eq!((10, 20), clock.as_tuple())
    }
    #[test]
    fn test_add_minutes_1() {
        let c = Clock::new(1, 1);
        let new_c = c.add_minutes(2);
        // assert_eq!(new_c.hours, 1);
        // assert_eq!(new_c.minutes, 3);
        assert_eq!(format!("{}", new_c), "01:03");
    }

    #[test]
    fn test_add_minutes_1_minus() {
        let c = Clock::new(5, 5);
        let new_c = c.add_minutes(-2);
        assert_eq!(new_c.hours, 5);
        assert_eq!(new_c.minutes, 3);
    }

    #[test]
    fn test_add_minutes_2() {
        let c = Clock::new(1, 1);
        let new_c = c.add_minutes(62);

        assert_eq!(new_c.hours, 2);
        assert_eq!(new_c.minutes, 3);
    }

    #[test]
    fn test_add_minutes_2_minus() {
        let c = Clock::new(3, 1);
        let new_c = c.add_minutes(-62);

        assert_eq!(new_c.hours, 1);
        assert_eq!(new_c.minutes, 59);
    }

    #[test]
    fn test_add_minutes_3() {
        let c = Clock::new(1, 1);
        let new_c = c.add_minutes((60 * 24) + (60 * 2) + 12);
        assert_eq!(new_c.hours, 3);
        assert_eq!(new_c.minutes, 13);
    }
    #[test]
    fn test_add_minutes_3_minus_1() {
        let c = Clock::new(1, 1);
        let new_c = c.add_minutes((-1) * ((60 * 24) + (60 * 2)));
        assert_eq!(new_c.hours, 23);
        assert_eq!(new_c.minutes, 1);
    }
    #[test]
    fn test_add_minutes_3_minus_2() {
        let c = Clock::new(1, 1);
        let new_c = c.add_minutes((-1) * ((60 * 24) + (60 * 2) + 12));
        assert_eq!(new_c.hours, 22);
        assert_eq!(new_c.minutes, 49);
    }
    #[test]
    fn test_add_across_midnight() {
        let clock = Clock::new(23, 59).add_minutes(2);
        assert_eq!(clock.to_string(), "00:01");
    }
    #[test]
    fn test_add_across_midnight_3hours() {
        let c = Clock::new(22, 45);
        let new_c = c.add_minutes(3 * 60 + 25);
        assert_eq!(format!("{}", new_c), "02:10");
    }
    #[test]
    fn test_add_across_midnight_minus() {
        let c = Clock::new(02, 10);
        let new_c = c.add_minutes((-1) * (3 * 60 + 25));
        assert_eq!(format!("{}", new_c), "22:45");
    }

    #[test]
    fn test_compare_clocks_with_hour_overflow() {
        assert_eq!(Clock::new(10, 37), Clock::new(34, 37));
    }

    #[test]
    fn test_compare_clocks_with_negative_hour_that_wraps() {
        assert_eq!(Clock::new(-31, 3), Clock::new(17, 3));
    }
}
