pub trait View {
    fn show_cards(&self, cards: Vec<(String, String, String)>);
    fn show_own_score(&self, score: usize);
    fn show_cardstack_top_card(&self, top_card: (String, String, String));
    fn show_cardstack_score(&self, score: usize);
    fn show_all_players_score(&self, players_score: (&str, usize));
    fn show_remaining_cards(&self, amount_remaining_cards: usize);

    fn get_user_input(&self) -> Result<String, String>;

}