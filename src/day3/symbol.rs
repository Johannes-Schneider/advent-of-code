static ASCII_STAR: u8 = 42;
static ASCII_DOT: u8 = 46;
static ASCII_NUMBER_0: u8 = 48;
static ASCII_NUMBER_9: u8 = ASCII_NUMBER_0 + 9;

#[derive(Debug, PartialEq)]
pub enum Symbol {
    Number(u8),
    Dot,
    Other(u8),
}

impl Symbol {
    pub fn parse_all(input: &[u8]) -> Vec<Symbol> {
        input.iter().map(|s| Symbol::parse(*s)).collect()
    }

    pub fn parse(input: u8) -> Symbol {
        if input == ASCII_DOT {
            return Symbol::Dot;
        }

        if input >= ASCII_NUMBER_0 && input <= ASCII_NUMBER_9 {
            return Symbol::Number(input - ASCII_NUMBER_0);
        }

        return Symbol::Other(input);
    }

    pub fn is_gear(&self) -> bool {
        match self {
            Symbol::Other(v) => *v == ASCII_STAR,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Symbol, ASCII_DOT};

    #[test]
    fn test_parse() {
        assert_eq!(Symbol::parse(ASCII_DOT), Symbol::Dot);
        assert_eq!(Symbol::parse(42), Symbol::Other(42));
        assert_eq!(Symbol::parse(35), Symbol::Other(35)); // that's a '#'
        assert_eq!(Symbol::parse(48), Symbol::Number(0));
        assert_eq!(Symbol::parse(49), Symbol::Number(1));
        assert_eq!(Symbol::parse(50), Symbol::Number(2));
        assert_eq!(Symbol::parse(51), Symbol::Number(3));
        assert_eq!(Symbol::parse(52), Symbol::Number(4));
        assert_eq!(Symbol::parse(53), Symbol::Number(5));
        assert_eq!(Symbol::parse(54), Symbol::Number(6));
        assert_eq!(Symbol::parse(55), Symbol::Number(7));
        assert_eq!(Symbol::parse(56), Symbol::Number(8));
        assert_eq!(Symbol::parse(57), Symbol::Number(9));
    }

    #[test]
    fn test_is_gear() {
        assert_eq!(Symbol::Number(0).is_gear(), false);
        assert_eq!(Symbol::Number(1).is_gear(), false);
        assert_eq!(Symbol::Number(2).is_gear(), false);
        assert_eq!(Symbol::Number(3).is_gear(), false);
        assert_eq!(Symbol::Number(4).is_gear(), false);
        assert_eq!(Symbol::Number(5).is_gear(), false);
        assert_eq!(Symbol::Number(6).is_gear(), false);
        assert_eq!(Symbol::Number(7).is_gear(), false);
        assert_eq!(Symbol::Number(8).is_gear(), false);
        assert_eq!(Symbol::Number(9).is_gear(), false);
        assert_eq!(Symbol::Dot.is_gear(), false);
        assert_eq!(Symbol::Other(35).is_gear(), false); // '#'
        assert_eq!(Symbol::Other(42).is_gear(), true); // '#'
    }
}
