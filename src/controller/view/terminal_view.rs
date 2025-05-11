use std::io;
use std::str::FromStr;

pub struct TerminalView {



}

impl TerminalView {

    pub fn new() -> Self {
        Self {}
    }

    pub fn show_current_player(&self, player_name: &str) {
        println!("Current player: {}", player_name);
    }

    pub fn show_card_hand(&self, card_name: Vec<&str>) {
        println!("Player cards:");

        for card in card_name.iter() {
            println!("  {}", card);
        }
    }

    pub fn show_current_counter(&self, counter: u32) {
        println!("Current counter: {}", counter);
    }

    pub fn get_user_input<T: FromStr>(&self) -> Result<T, <T as FromStr>::Err> {
        let mut input: String = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Input could not be read");

        input.trim().parse()
    }

}