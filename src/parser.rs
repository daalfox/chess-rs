use std::str::FromStr;

use super::Square;
impl FromStr for Square {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();
        if chars.len() != 2 {
            return Err(());
        }

        let file = match chars[0] {
            f @ ('a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h') => f,
            _ => return Err(()),
        };
        let rank = match chars[1] {
            r @ ('1' | '2' | '3' | '4' | '5' | '6' | '7' | '8') => r as u8,
            _ => return Err(()),
        };

        Ok(Self(file, rank))
    }
}
impl TryFrom<(char, u8)> for Square {
    type Error = ();
    fn try_from(value: (char, u8)) -> Result<Self, Self::Error> {
        let file = match value.0 {
            f @ ('a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h') => f,
            _ => return Err(()),
        };
        let rank = match value.1 {
            r @ (1 | 2 | 3 | 4 | 5 | 6 | 7 | 8) => r,
            _ => return Err(()),
        };

        Ok(Square(file, rank))
    }
}
