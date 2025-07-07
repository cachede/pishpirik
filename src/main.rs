mod ecs;
mod network;
use std::collections::HashMap;
use std::process::exit;

use rand::{seq::SliceRandom};
use crate::ecs::Components;
use colored::Colorize;
use rand::thread_rng;
use std::sync::atomic::{AtomicUsize, Ordering};

type Entities = HashMap<&'static str, Vec<HashMap<&'static str, ecs::Components>>>;
type Cards = Vec<HashMap<&'static str, ecs::Components>>;

const DISCARD_PILE_MIN_CAPACITY: usize = 4;
const GAMES: &'static str = "games";
const NEW_TURN: &'static str = "new turn";
const ACTIVE_PLAYER: &'static str = "active player";
const DISCARD_PILE: &'static str = "discard pile";
const DRAW_PILE: &'static str = "draw pile";
const COLOR: &'static str = "color";
const VALUE: &'static str = "value";
const PLAYERS: &'static str = "players";
const HAND: &'static str = "hand";
const PLAYER_NUMBER: &'static str = "number";
const NAME: &'static str = "name";
const STASH: &'static str = "stash";
const POINTS: &'static str = "points";

const CLUBS: &'static str = "clubs";
const DIAMONDS: &'static str = "diamonds";
const HEARTS: &'static str = "hearts";
const SPADES: &'static str = "spades";

const TWO: &'static str = "2";
const THREE: &'static str = "3";
const FOUR: &'static str = "4";
const FIVE: &'static str = "5";
const SIX: &'static str = "6";
const SEVEN: &'static str = "7";
const EIGHT: &'static str = "8";
const NINE: &'static str = "9";
const TEN: &'static str = "10";
const JACK: &'static str = "Jack";
const QUEEN: &'static str = "Queen";
const KING: &'static str = "King";
const ACE: &'static str = "Ace";

const AMOUNT_PLAYERS: usize = 4;
const AMOUNT_OF_CARDS_PER_PLAYER: usize = 4;

fn fill_discard_pile(entities: &mut Entities, cards: &mut Cards) -> Option<()> {
    let games = entities.get_mut(GAMES)?;
    let game = games.get_mut(0)?;
    let discard_pile = match game.get_mut(DISCARD_PILE) {
        Some(ecs::Components::V(vec)) => vec,
        _ => return None,
    };
    let mut index = 0;

    while discard_pile.len() < DISCARD_PILE_MIN_CAPACITY {
        let card1 = cards.get_mut(index)?;
        let mut is_not_jack = false;

        if let (ecs::Components::S(pic), ecs::Components::S(color)) =
            (card1.get(VALUE)?, card1.get(COLOR)?)
        {
            if *pic != "Jack" {
                is_not_jack = true;
                println!(
                    "next card is {} of {}, putting on top of discard pile\r",
                    pic, color
                );
            } else {
                println!(
                    "next card is {} of {}, not putting on top of discard pile\r",
                    pic, color
                );
            }

            if is_not_jack {
                discard_pile.push(cards.remove(index));
            }

            index += 1;
        }
    }

    println!("\r");
    println!("discard pile is now {} big", discard_pile.len());
    println!("\r");

    Some(())
}

fn fill_draw_pile(entities: &mut Entities, cards: &mut Cards) -> Option<()> {
    let games = entities.get_mut(GAMES)?;
    let game = games.get_mut(0)?;
    let draw_pile = match game.get_mut(DRAW_PILE) {
        Some(ecs::Components::V(vec)) => vec,
        _ => return None,
    };

    draw_pile.append(cards);

    println!("\r");
    println!("draw pile is now {} big", draw_pile.len());
    println!("\r");

    Some(())
}

fn create_player(player: &mut HashMap<&'static str, ecs::Components>, player_name: &'static str) {
    static PLAYER_ID: AtomicUsize = AtomicUsize::new(1);

    player.insert(NAME, ecs::Components::S(player_name));
    player.insert(PLAYER_NUMBER, ecs::Components::I(PLAYER_ID.fetch_add(1, Ordering::Relaxed)));
    player.insert(HAND, ecs::Components::V(vec![]));
    player.insert(STASH, ecs::Components::V(vec![]));
    player.insert(POINTS, ecs::Components::I(0));
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut entities = ecs::new_entities_repo();
    let mut systems = ecs::new_systems_repo();

    let mut player1 = ecs::create_new_entity();
    create_player(&mut player1, "player1");
    ecs::add_entity_to_group(&mut entities, player1, PLAYERS);

    let mut player2 = ecs::create_new_entity();
    create_player(&mut player2, "player2");
    ecs::add_entity_to_group(&mut entities, player2, PLAYERS);

    let mut player3 = ecs::create_new_entity();
    create_player(&mut player3, "player3");
    ecs::add_entity_to_group(&mut entities, player3, PLAYERS);

    let mut player4 = ecs::create_new_entity();
    create_player(&mut player4, "player4");
    ecs::add_entity_to_group(&mut entities, player4, PLAYERS);

    let mut game = ecs::create_new_entity();
    game.insert(ACTIVE_PLAYER, ecs::Components::I(1));
    game.insert(DISCARD_PILE, ecs::Components::V(vec![]));
    game.insert(DRAW_PILE, ecs::Components::V(vec![]));
    game.insert(NEW_TURN, ecs::Components::B(true));
    ecs::add_entity_to_group(&mut entities, game, GAMES);

    let mut cards: Cards = create_deck();

    cards.shuffle(&mut thread_rng());

    match fill_discard_pile(&mut entities, &mut cards) {
        Some(()) => {println!("FILLED DISCARD PILE")}
        None => {
            return Err("Failed to fill the discard pile".into());
        }
    }

    match fill_draw_pile(&mut entities, &mut cards) {
        Some(()) => {}
        None => {
            return Err("Failed to fill the draw pile".into());
        }
    }

    ecs::add_system(&mut systems, exit_game_system);
    ecs::add_system(&mut systems, draw_cards_system);
    ecs::add_system(&mut systems, play_cards_system);
    ecs::add_system(&mut systems, new_turn_system);

    //ecs::enable_input();

    let mut server = network::GameServer::new("0.0.0.0:37545").unwrap(); // hier unwrap weil wir im setup das programm beenden wollen wenn der server nicht gestartet werden kann

    loop{

        // der server wird gepolled, alle nachrichten werden verarbeitet
        match server.poll() {
            Ok(_) => () /*println!("polled server successfully")*/,
            Err(e) => () /*println!("error polling server: {}", e)*/
        }
        // ein leerer input buffer wird vorbereitet, falls kein spieler gefunden wird wird der leere input buffer verwendet (zb wenn noch nicht jeder verbunden ist)
        let mut input_buffer = HashMap::new();
        for key in 1..=4 {
            input_buffer.insert(key, false);
        }

        // der input buffer vom aktiven spieler wird genommen
        if let Some(games) = entities.get(GAMES) {
            if let Some(game) = games.get(0) {
                if let Some(ecs::Components::I(active_player)) = game.get(ACTIVE_PLAYER) {
                    let player_index = (*active_player as usize).saturating_sub(1);
                    if player_index < server.players.len() {
                        input_buffer = server.players[player_index].input_buffer.clone();
                    } else {
                        /*println!("Player {} not connected (have {} players)",
                                 active_player, server.players.len());*/
                    }
                }
            }
        }

        // alle input buffer werden zurück gesetzt
        for player in &mut server.players {
            for key in 1..=4 {
                player.input_buffer.insert(key, false);
            }
        }

        ecs::process(&mut entities, &systems, &input_buffer);

        match server.send_entities(&entities) {
            Ok(_) => ()/*println!("successfully sent entities to all clients")*/,
            Err(e) => ()/*println!("failed to send entities to all clients: {}", e)*/
        }

        std::thread::sleep(std::time::Duration::from_millis(50));
    }

}

fn all_hands_empty(entities: &Entities) -> bool {
    let mut all_hands_empty = true;

    if let Some(players) = entities.get(PLAYERS) {
        for player in players {
            if let Some(ecs::Components::V(hand)) = player.get("hand") {
                if !hand.is_empty() {
                    all_hands_empty = false
                }
            }
        }
    }

    all_hands_empty
}

fn update_turn(entities: &mut Entities) -> Option<()> {
    let mut games = entities.get_mut(GAMES)?;
    let mut game = games.get_mut(0)?;

    if let Some(ecs::Components::B(new_turn)) = game.get_mut("new turn") {
        if !*new_turn {
            return None;
        } else {
            *new_turn = false;
            println!("new turn has been processed");
        }
    }

    Some(())
}

fn get_active_player(entities: &mut Entities) -> Option<usize> {
    let games = entities.get_mut(GAMES)?;
    let game = games.get_mut(0)?;

    let active_player_index = match game.get(ACTIVE_PLAYER) {
        Some(Components::I(i)) => Some(*i),
        _ => None,
    };

    active_player_index
}

fn new_turn_system(entities: &mut Entities, _input: &HashMap<u8, bool>) -> Option<()> {
    if all_hands_empty(entities) {
        return None;
    }

    update_turn(entities)?;

    let active_player_index = get_active_player(entities)?;
    let players = entities.get(PLAYERS)?;
    let active_player = players.get(active_player_index - 1)?;

    let output = format!("it is now player{}s turn", active_player_index);
    println!("\r");
    println!("------------------------------\r");
    println!("\r");
    println!("{}", output.bold().green());
    println!("\r");
    println!("your cards are:\r");
    println!("\r");

    //print player-cards
    if let Some(Components::V(hand)) = active_player.get(HAND) {
        for (index, card) in hand.iter().enumerate() {
            if let (Some(value), Some(color)) = (card.get("value"), card.get("color")) {
                let output = format!("{}: {} of {}", index + 1, value, color);
                println!("{}\r", output.green());
            }
        }

        println!("\r");
    }

    //print top card
    let games = entities.get(GAMES)?;
    let game = games.get(0)?;
    // show top card in discard pile
    if let Some(ecs::Components::V(discard_pile)) = game.get(DISCARD_PILE) {
        if let Some(top_card) = discard_pile.last() {
            if let (Some(value), Some(color)) = (top_card.get("value"), top_card.get("color")) {
                let output = format!("top card is {} of {}", value, color);
                println!("{}\r", output.bold().blue());
                println!("\r");
            }
        }
    }

    Some(())
}

fn get_played_card(
    entities: &mut Entities,
    input: &HashMap<u8, bool>,
) -> Option<(HashMap<&'static str, Components>)> {
    let games = entities.get("games")?;
    let game = games.get(0)?;
    let active_player_index = match game.get(ACTIVE_PLAYER) {
        Some(Components::I(i)) => *i,
        _ => return None,
    };

    let players = entities.get_mut(PLAYERS)?;
    let active_player = players.get_mut(active_player_index - 1)?;
    let mut played_card: HashMap<&'static str, Components> = HashMap::new();

    if let Some(Components::V(hand)) = active_player.get_mut(HAND) {
        if !hand.is_empty() {
            let (one, two, three, four) = (
                input.get(&1u8).unwrap_or(&false),
                input.get(&2u8).unwrap_or(&false),
                input.get(&3u8).unwrap_or(&false),
                input.get(&4u8).unwrap_or(&false),
            );

            if *one && hand.len() >= 1 {
                played_card = hand.remove(0);
            } else if *two && hand.len() >= 2 {
                played_card = hand.remove(1);
            } else if *three && hand.len() >= 3 {
                played_card = hand.remove(2);
            } else if *four && hand.len() >= 4 {
                played_card = hand.remove(3);
            } else {
                return None;
            }

        }
    }

    Some(played_card)
}

fn change_turn(entities: &mut Entities) -> Option<()> {
    let games = entities.get_mut(GAMES)?;
    let game = games.get_mut(0)?;
    let active_player_i = match game.get_mut(ACTIVE_PLAYER) {
        Some(Components::I(current_player_index)) => current_player_index,
        _ => return None,
    };
    *active_player_i = (*active_player_i % 4) + 1;
    if let Some(Components::B(new_turn)) = game.get_mut(NEW_TURN) {
        *new_turn = true;
    }

    Some(())
}

fn played_card_add_points(
    entities: &mut Entities,
    player_index: usize,
    played_card_value: &&str,
) -> Option<()> {
    let mut players = entities.get_mut(PLAYERS)?;
    let mut active_player = players.get_mut(player_index)?;

    if let Some(ecs::Components::I(points)) = active_player.get_mut(POINTS) {
        if *played_card_value == ACE
            || *played_card_value == TEN
            || *played_card_value == KING
            || *played_card_value == QUEEN
            || *played_card_value == JACK
        {
            *points = *points + 20;
            println!("player{} scored 20 points\r", player_index);
        } else {
            *points = *points + 10;
            println!("player{} scored 10 points\r", player_index);
        }
    }

    Some(())
}

fn player_add_cardstack_points(
    entities: &mut Entities,
    player_index: usize,
    cardstack: &Vec<HashMap<&'static str, Components>>,
) -> Option<()> {
    let mut players = entities.get_mut(PLAYERS)?;
    let mut active_player = players.get_mut(player_index)?;

    if let Some(ecs::Components::I(points)) = active_player.get_mut(POINTS) {
        for card in cardstack {
            if let Some(ecs::Components::S(value)) = card.get("value") {
                if is_card_value_high_value(value) {
                    *points = *points + 1;
                    println!("player{} scored 1 point\r", player_index);
                }
            }
        }

        println!("\r");
        Some(())
    } else {
        println!("could not find component points in player{}", player_index);
        None
    }
}

fn play_cards_system(entities: &mut Entities, input: &HashMap<u8, bool>) -> Option<()> {
    let mut played_card: HashMap<&'static str, Components> = HashMap::new();

    let active_player_index = get_active_player(entities)?;

    // get played card
    played_card = get_played_card(entities, input)?;

    //print played card
    if let (Some(value), Some(color)) = (played_card.get("value"), played_card.get("color")) {
        let output = format!(
            "player{} has played {} of {}",
            active_player_index, value, color
        );
        println!("{}\r", output.bold());
    }

    let mut temporary_stack = {
        let mut games = entities.get_mut(GAMES)?;
        let mut game = games.get_mut(0)?;
        let discard_pile = match game.get_mut(DISCARD_PILE) {
            Some(Components::V(discard_pile)) => discard_pile,
            _ => return None,
        };
        std::mem::take(discard_pile)
    };

    // new turn
    change_turn(entities)?;

    if let Some(topcard) = temporary_stack.last() {

        let top_card_value = match topcard.get(VALUE) {
            Some(Components::S(top_card_value)) => top_card_value,
            _ => return None,
        };

        let played_card_value = match played_card.get(VALUE) {
            Some(Components::S(played_card_value)) => played_card_value,
            _ => return None,
        };

        // pishpirik spiellogik
        if played_card_value == top_card_value || *played_card_value == JACK {
            if played_card_value == top_card_value {
                println!(
                    "player{} played the same card as top card\r",
                    active_player_index
                );
            } else {
                println!("player{} played a Jack\r", active_player_index);
            }

            if temporary_stack.len() == 1 && played_card_value == top_card_value {
                println!("player{} got extra points\r", active_player_index);

                // add points to player
                played_card_add_points(entities, active_player_index - 1, played_card_value)?;
            }

            temporary_stack.push(played_card);

            println!("player{} took the discard pile\r", active_player_index);

            //add player points
            player_add_cardstack_points(entities, active_player_index - 1, &temporary_stack)?;
            //add cards to player stash
            add_stash_to_player(entities, active_player_index - 1, &mut temporary_stack);

            //print player stash
            print_player_stash(entities, active_player_index)?;

            return Some(());
        }
    }

    temporary_stack.push(played_card);

    let mut games = entities.get_mut(GAMES)?;
    let mut game = games.get_mut(0)?;
    game.insert("discard pile", ecs::Components::V(temporary_stack));

    Some(())
}

fn print_player_stash(entities: &mut Entities, player_index: usize) -> Option<()> {
    let players = entities.get(PLAYERS)?;
    let active_player = players.get(player_index)?;

    if let Some(ecs::Components::V(stash)) = active_player.get(STASH) {
        println!("player{}s stash is now:\r", player_index);

        for card in stash {
            if let (Some(value), Some(color)) = (card.get(VALUE), card.get(COLOR)) {
                let output = format!("{} of {}", value, color);
                println!("{}\r", output.italic());
            }
        }
    } else {
        println!("error could not find player{}s stash", player_index);
        return None;
    }

    //print player total points
    if let Some(ecs::Components::I(points)) = active_player.get(POINTS) {
        println!("player{}s total points are now {}\r", player_index, points);
        Some(())
    } else {
        None
    }
}

fn add_stash_to_player(
    entities: &mut Entities,
    player_index: usize,
    temporary_stash: &mut Vec<HashMap<&'static str, Components>>,
) -> Option<()> {
    let mut players = entities.get_mut(PLAYERS)?;
    let mut active_player = players.get_mut(player_index)?;

    if let Some(ecs::Components::V(stash)) = active_player.get_mut(STASH) {
        stash.append(temporary_stash);
        Some(())
    } else {
        None
    }
}

fn draw_cards_system(entities: &mut Entities, _input: &HashMap<u8, bool>) -> Option<()> {
    if all_hands_empty(entities) {
        let mut drawn_cards: Vec<HashMap<&'static str, ecs::Components>> = vec![];
        let mut draw_pile_size = 0;
        let games = entities.get_mut(GAMES)?;
        let game = games.get_mut(0)?;
        let draw_pile = match game.get_mut(DRAW_PILE) {
            Some(Components::V(draw_pile)) => draw_pile,
            _ => return None,
        };


        for _ in 0..(AMOUNT_PLAYERS * AMOUNT_OF_CARDS_PER_PLAYER) {
            if let Some(card) = draw_pile.pop() {
                drawn_cards.push(card);
            } else {
                let output = format!("all cards have been played");
                println!("{}\r", output.bold().red());
                println!("\r");
                return Some(());
            }
        }

        draw_pile_size = draw_pile.len();


        let output = format!("all hands are empty, everyone will draw 4 cards");
        println!("{}\r", output.bold().yellow());


        let players = entities.get_mut(PLAYERS)?;

        // give players cards
        for player in players.iter_mut() {
            let player_hand = match player.get_mut(HAND) {
                Some(Components::V(player_hand)) => player_hand,
                _ => return None,
            };
            for _ in 0..AMOUNT_OF_CARDS_PER_PLAYER {
                if let Some(card) = drawn_cards.pop() {
                    player_hand.push(card);
                }
            }
        }

        // print drawn player cards
        for player in players.iter_mut() {
            println!("\r");

            let player_hand = match player.get(HAND) {
                Some(Components::V(player_hand)) => player_hand,
                _ => return None,
            };

            for card in player_hand {
                if let (Some(name), Some(value), Some(color)) =
                    (player.get(NAME), card.get(VALUE), card.get(COLOR))
                {
                    let output = format!("{} has drawn {} of {}\r", name, value, color);
                    println!("{}", output.italic());
                }
            }
        }

        println!("\r");
        println!("draw pile now has {} cards left\r", draw_pile_size);
        println!("\r");

    }
    Some(())
}

fn exit_game(entities: &mut HashMap<&'static str, Vec<HashMap<&'static str, ecs::Components>>>) {
    println!("\r");
    let output = format!("the game is now over");
    println!("{}\r", output.bold().red());
    println!("\r");

    if let Some(players) = entities.get(PLAYERS) {
        for player in players {
            if let Some(number) = player.get("number") {
                let output = format!("player{}s stash:", number);
                println!("{}\r", output.bold().blue())
            }

            if let Some(ecs::Components::V(stash)) = player.get(STASH) {
                for card in stash {
                    if let (Some(value), Some(color)) = (card.get(VALUE), card.get(COLOR)) {
                        println!("{} of {}\r", value, color);
                    }
                }
            }

            if let Some(ecs::Components::I(points)) = player.get(POINTS) {
                let output = format!("total points: {}", points);
                println!("{}\r", output.bold().blue());
                println!("\r");
            }
        }
    }

    //ecs::disable_input();
    exit(0);
}

fn exit_game_system(entities: &mut Entities, input: &HashMap<u8, bool>) -> Option<()> {
    let mut draw_pile_empty = false;
    let mut games = entities.get_mut(GAMES)?;
    let mut game = games.get_mut(0)?;
    let draw_pile = match game.get_mut(DRAW_PILE) {
        Some(Components::V(draw_pile)) => draw_pile,
        _ => return None,
    };

    if draw_pile.is_empty() && all_hands_empty(entities) {
        exit_game(entities);
    }

//    if let Some(back) = input.get("back"){
//
//        if *back{
//
//            exit_game(entities);
//
//        }
//
//    }

    Some(())
}

fn create_deck() -> Vec<HashMap<&'static str, ecs::Components>> {
    let mut cards = vec![];
    let types = vec![CLUBS, DIAMONDS, HEARTS, SPADES];
    let values = vec![TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE, TEN, JACK, QUEEN, KING, ACE];

    let mut counter = 0;

    for color in &types {
        for value in &values {
            let mut new_card = ecs::create_new_entity();
            new_card.insert(COLOR, ecs::Components::S(color));
            new_card.insert(VALUE, ecs::Components::S(value));
            cards.push(new_card);

            println!("generated {} of {}\r", value, color);

            counter = counter + 1;
        }
    }

    println!("\r");
    println!("generated {} cards in total", counter);
    println!("\r");

    cards
}

fn is_card_value_high_value(value: &'static str) -> bool {
    value == ACE || value == TEN || value == KING || value == QUEEN || value == JACK
}
