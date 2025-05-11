pub struct Model{
    // alle Spieler in einem vec
    // Kartenstapel
    // Wer grade dran ist
    pub counter: u32
}

impl Model {

    pub fn new(counter: u32) -> Model {
        Self{
            counter: 0
        }
    }

    pub fn increment(&mut self) {
        self.counter += 1;
    }

}