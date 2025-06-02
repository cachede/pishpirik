use crate::controller::model::card::Card;

pub struct Player {
    name: String,
    cards: Vec<Option<Card>>,
    score: usize,
}


impl Player {
    pub fn new(name: String) -> Self {
        Self {
            name,
            cards: vec![],
            score: 0,
        }
    }

    pub fn add_card(&mut self, card: Card) {

        if self.cards.len() < 4 {
            self.cards.push(Some(card));
        }
    }

    pub fn clear_hand(&mut self) {
        self.cards.clear();
    }

    pub fn dispose_card(&mut self, index: usize) -> Option<Card> {

        self.cards.get_mut(index).and_then(|x| x.take())

    }

    pub fn get_cards(&self) -> &Vec<Option<Card>> {
        &self.cards
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

}

#[cfg(test)]
mod tests {
    use crate::controller::model::card::Card;
    use crate::controller::model::card::{CardColor, CardFace, CardSuit};
    use crate::controller::model::player::Player;

    #[test]
    fn get_player_name() {
        let player = Player::new("Test".to_string());

        assert_eq!(player.name, "Test");
    }

    #[test]
    fn add_card() {
        let mut player = Player::new("Test".to_string());
        player.add_card(Card::new(CardFace::ASS, CardSuit::DIAMOND, CardColor::RED));

        println!("Player card size: {}", player.cards.len());

        assert_eq!(player.cards.len(), 1);
    }

    #[test]
    fn add_card_and_dispose() {
        let mut player = Player::new("Test".to_string());
        player.add_card(Card::new(CardFace::ASS, CardSuit::DIAMOND, CardColor::RED));
        assert_eq!(player.cards.len(), 1);
        let card = player.dispose_card(0);
        assert_eq!(card.is_some(), true);
        assert_eq!(card.unwrap().card_face, CardFace::ASS);
        assert_eq!(player.cards.get(0).unwrap().is_none(), true)
    }

}