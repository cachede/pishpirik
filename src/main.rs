use crate::controller::controller::{Controller, ViewType};

mod controller;

fn main() {

    let mut controller = Controller::new(ViewType::TerminalView);
    controller.run();

}
