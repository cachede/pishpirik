use crate::controller::controller::Controller;

mod controller;

fn main() {

    let mut controller = Controller::new();
    controller.run();

}
