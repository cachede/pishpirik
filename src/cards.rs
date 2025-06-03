use std::fmt;

pub enum VALUES{
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    TEN,
    JACK,
    QUEEN,
    KING,
    ACE,
}

impl fmt::Display for VALUES {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VALUES::TWO => write!(f, "2"),
            VALUES::THREE => write!(f, "3"),
            VALUES::FOUR => write!(f, "4"),
            VALUES::FIVE => write!(f, "5"),
            VALUES::SIX => write!(f, "6"),
            VALUES::SEVEN => write!(f, "7"),
            VALUES::EIGHT => write!(f, "8"),
            VALUES::NINE => write!(f, "9"),
            VALUES::TEN => write!(f, "10"),
            VALUES::JACK => write!(f, "Jack"),
            VALUES::QUEEN => write!(f, "Queen"),
            VALUES::KING => write!(f, "King"),
            VALUES::ACE => write!(f, "Ace"),
        }
    }
}

pub enum COLORS{
    CLUBS,
    DIAMONDS,
    HEARTS,
    SPADES,
}

impl fmt::Display for COLORS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            COLORS::CLUBS => write!(f, "Clubs"),
            COLORS::DIAMONDS => write!(f, "Diamonds"),
            COLORS::HEARTS => write!(f, "Hearts"),
            COLORS::SPADES => write!(f, "Spades"),
        }
    }
}
