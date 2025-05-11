use super::model::model::Model;
use super::view::terminal_view::TerminalView;


pub struct Controller {
    model: Model,
    view: TerminalView,
    running: bool
}

impl Controller {

    // TODO: DI for view
    pub fn new() -> Controller {
        Self {
            model: Model::new(0),
            view: TerminalView::new(),
            running: true
        }
    }

    pub fn run(&mut self) {

        while self.running {

            self.view.show_current_counter(self.model.counter);

            let user_input = loop {

                match self.view.get_user_input::<String>() {
                    Ok(input) => break input,
                    Err(_) => println!("Please try again"),
                }
            };


            match user_input.as_str() {
                "q" => {
                    self.running = false
                },
                "1" => {
                    self.model.increment();
                },
                _ => println!("Unknown input: {}", user_input),
            }

        }
    }




}