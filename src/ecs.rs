use std::fmt;
use std::collections::HashMap;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

pub enum Components {

    B(bool),
    I(i32),
    S(&'static str),
    V(Vec<HashMap<&'static str, Components>>)

}

impl fmt::Display for Components {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Components::B(b) => write!(f, "{}", b),
            Components::I(i) => write!(f, "{}", i),
            Components::S(s) => write!(f, "{}", s),
            Components::V(_) => write!(f, "vector")
        }
    }
}

pub fn new_entities_repo() -> HashMap<&'static str, Vec<HashMap<&'static str, Components>>> {
    HashMap::new()
}

pub fn add_entity_to_group(
    entities: &mut HashMap<&'static str, Vec<HashMap<&'static str, Components>>>,
    entity: HashMap<&'static str, Components>,
    group: &'static str
){
    let entity_list = entities.entry(group).or_insert_with(Vec::new);

    entity_list.push(entity);
}

pub fn create_new_entity() -> HashMap<&'static str, Components>{
    HashMap::new()
}

pub fn new_systems_repo() -> Vec<fn(&mut HashMap<&'static str, Vec<HashMap<&'static str, Components>>>, &HashMap<&'static str, bool>)> {
    vec![]
}

pub fn add_system(
    systems: &mut Vec<fn(&mut HashMap<&'static str, Vec<HashMap<&'static str, Components>>>, &HashMap<&'static str, bool>)>,
    system: fn(&mut HashMap<&'static str, Vec<HashMap<&'static str, Components>>>, &HashMap<&'static str, bool>)
){
    systems.push(system);
}

pub fn process(
    entities: &mut HashMap<&'static str, Vec<HashMap<&'static str, Components>>>,
    systems: &Vec<fn(&mut HashMap<&'static str, Vec<HashMap<&'static str, Components>>>, &HashMap<&'static str, bool>)>
){
    let current_input = get_input();
    for system in systems{

        system(entities, &current_input);

    }
}

pub fn enable_input(){
    enable_raw_mode().ok();
}

pub fn disable_input(){
    disable_raw_mode().ok();
}

pub fn get_input() -> HashMap<&'static str, bool> {

    let mut input_state = HashMap::from([

        ("left", false),
        ("right", false),
        ("up", false),
        ("down", false),
        ("space", false),
        ("backspace", false),
        ("1", false),
        ("2", false),
        ("3", false),
        ("4", false),

    ]);

    while event::poll(std::time::Duration::from_millis(0)).unwrap_or(false) {

        if let Ok(Event::Key(KeyEvent { code, .. })) = event::read() {

            match code {

                KeyCode::Left => input_state.insert("left", true),
                KeyCode::Right => input_state.insert("right", true),
                KeyCode::Up => input_state.insert("up", true),
                KeyCode::Down => input_state.insert("down", true),
                KeyCode::Char(' ') => input_state.insert("space", true),
                KeyCode::Backspace => input_state.insert("back", true),
                KeyCode::Char('1') => input_state.insert("1", true),
                KeyCode::Char('2') => input_state.insert("2", true),
                KeyCode::Char('3') => input_state.insert("3", true),
                KeyCode::Char('4') => input_state.insert("4", true),
                _ => None,

            };

        }

    }

    input_state

}
