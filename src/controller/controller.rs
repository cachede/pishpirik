use super::model::model::Model;
use super::view::terminal_view::TerminalView;
use super::view::view_trait::View;

#[derive(PartialEq)]
pub enum ViewType {
    TerminalView,
    GuiView,
}

pub struct Controller {
    model: Model,
    view: Box<dyn View>,
    running: bool
}

impl Controller {

    pub fn new(view_type: ViewType) -> Controller {

        if view_type == ViewType::TerminalView {
            Self {
                model: Model::new(0),
                view: Box::new(TerminalView::new()),
                running: true
            }
        } else {
            Self {
                model: Model::new(0),
                view: Box::new(TerminalView::new()),  //TODO: change this to GuiView
                running: true
            }
        }
    }

    pub fn run(&mut self) {

        while self.running {

            self.view.show_cards(vec![("K".to_string(), "♦".to_string(), "RED".to_string())]);
            self.view.show_own_score(/*current_player.score*/ 0);
            self.view.show_cardstack_top_card(("K".to_string(), "♦".to_string(), "RED".to_string()));
            self.view.show_cardstack_score(/*carstack.score */ 0);
            self.view.show_all_players_score(/*player tuple with name | count*/ ("Player 1", 0));
            self.view.show_remaining_cards(/*cardstack.remaining_cards */ 0);

            let user_input = loop {

                match self.view.get_user_input() {
                    Ok(input) => break input,
                    Err(_) => println!("Please try again"),
                }
            };


            match user_input.as_str() {
                "q" => {
                    self.running = false
                },
                _ => println!("Unknown input: {}", user_input),
            }

        }
    }




}