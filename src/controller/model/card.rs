
#[derive(Debug, Eq, PartialEq)]
pub enum CardColor {
    RED,
    BLACK
}

#[derive(Debug, Eq, PartialEq)]
pub enum CardSuit {
    CLUB,
    DIAMOND,
    HEART,
    SPADE
}

#[derive(Debug, Eq, PartialEq)]
pub enum CardFace {
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    TEN,
    BUBE,
    DAME,
    KOENIG,
    ASS
}


// TODO remove the CardColor => CardSuite is indirectly the color
#[derive(Debug, Eq, PartialEq)]
pub struct Card {

    pub card_face: CardFace,
    card_color: CardColor,
    card_suit: CardSuit,

}

impl Card {

    pub fn new(card_face: CardFace, card_suit: CardSuit, card_color: CardColor) -> Self {
        Self {
            card_face,
            card_color,
            card_suit,

        }
    }

    //TODO: für jedes feld in struct eine to_string() methode (wichtig für view)

}