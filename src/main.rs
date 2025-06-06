mod ecs;
mod cards;
use std::collections::HashMap;
use std::fmt::{self, write};
use std::process::exit;

use rand::{seq::SliceRandom, Fill};
use rand::thread_rng;
use colored::Colorize;
use std::sync::atomic::{AtomicI32, Ordering};

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

fn fill_discard_pile(entities: &mut Entities, cards: &mut Cards) -> Option<()> {
    let games = entities.get_mut(GAMES)?;
    let game = games.get_mut(0)?;
    let discard_pile = match game.get_mut(DISCARD_PILE) {
        Some(ecs::Components::V(vec)) => vec,
        _ => return None
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
                println!("next card is {} of {}, putting on top of discard pile\r", pic, color);

            } else {

                println!("next card is {} of {}, not putting on top of discard pile\r", pic, color);
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
        _ => return None
    };

    draw_pile.append(cards);

    println!("\r");
    println!("draw pile is now {} big", draw_pile.len());
    println!("\r");

    Some(())
}

fn create_player(player: &mut HashMap<&'static str, ecs::Components>, player_name: &'static str) {

    static PLAYER_ID: AtomicI32 = AtomicI32::new(1);

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
        Some(()) => {}
        None => {return Err("Failed to fill the discard pile".into());}
    }

    match fill_draw_pile(&mut entities, &mut cards) {
        Some(()) => {}
        None => {return Err("Failed to fill the draw pile".into());}
    }

    println!("CARDS VEC SOLLTE HIER 0 SEIN {}", cards.len());

    ecs::add_system(&mut systems, exit_game_system);
    ecs::add_system(&mut systems, draw_cards_system);
    ecs::add_system(&mut systems, play_cards_system);
    ecs::add_system(&mut systems, new_turn_system);

    ecs::enable_input();

    loop{

        ecs::process(&mut entities, &systems);

    }

}

fn all_hands_empty(entities: &Entities) -> bool {

    let mut all_hands_empty = true;

    if let Some(players) = entities.get(PLAYERS){

        for player in players{

            if let Some(ecs::Components::V(hand)) = player.get(HAND){

                if !hand.is_empty(){

                    all_hands_empty = false

                }

            }

        }

    }

    all_hands_empty

}

fn new_turn_system(entities: &mut Entities, _input: &HashMap<&'static str, bool>){

    if all_hands_empty(entities){
        return;
    }

    if let Some(games) = entities.get_mut(GAMES){

        if let Some(game) = games.get_mut(0){

            if let Some(ecs::Components::B(new_turn)) = game.get_mut(NEW_TURN){

                if !*new_turn{

                    return;

                } else {

                    *new_turn = false;

                }

            }

        }

    }

    let mut active_player: i32 = 1;

    if let Some(games) = entities.get(GAMES){

        if let Some(game) = games.get(0){

            if let Some(ecs::Components::I(active_player_temp)) = game.get(ACTIVE_PLAYER){

                active_player = *active_player_temp;

            }

        }

    }

    let output = format!("it is now player{}s turn", active_player);
    println!("\r");
    println!("------------------------------\r");
    println!("\r");
    println!("{}", output.bold().green());
    println!("\r");
    println!("your cards are:\r");
    println!("\r");

    if let Some(players) = entities.get(PLAYERS){

        for player in players{

            if let Some(ecs::Components::I(number)) = player.get(PLAYER_NUMBER){

                if *number == active_player{

                    if let Some(ecs::Components::V(hand)) = player.get(HAND){

                        for (index, card) in hand.iter().enumerate() {

                            if let (Some(value), Some(color)) = (card.get(VALUE), card.get(COLOR)) {

                                let output = format!("{}: {} of {}", index+1, value, color);
                                println!("{}\r", output.green());

                            }

                        }

                        println!("\r");

                    }

                }

            }

        }

    }

    if let Some(games) = entities.get(GAMES){

        if let Some(game) = games.get(0){

            if let Some(ecs::Components::V(discard_pile)) = game.get(DISCARD_PILE){

                if let Some(top_card) = discard_pile.last(){

                    if let (Some(value), Some(color)) = (top_card.get(VALUE), top_card.get(COLOR)){

                        let output = format!("top card is {} of {}", value, color);
                        println!("{}\r", output.bold().blue());
                        println!("\r");

                    }

                }

            }

        }

    }

}

fn play_cards_system(entities: &mut Entities, input: &HashMap<&'static str, bool>){

    let mut active_player: i32 = 1;
    let mut temporary_stack: Vec<HashMap<&'static str, ecs::Components>> = vec![];
    let mut played_card: HashMap<&'static str, ecs::Components> = HashMap::new();

    if let Some(games) = entities.get(GAMES){

        if let Some(game) = games.get(0){

            if let Some(ecs::Components::I(active_player_temp)) = game.get(ACTIVE_PLAYER){

                active_player = *active_player_temp;

            }

        }

    }

    if let Some(players) = entities.get_mut(PLAYERS){

        for player in players{

            if let Some(ecs::Components::I(number)) = player.get(PLAYER_NUMBER){

                if *number == active_player{

                    if let Some(ecs::Components::V(hand)) = player.get_mut(HAND){

                        if !hand.is_empty(){

                            if let (Some(one), Some(two), Some(three), Some(four)) = (input.get("1"), input.get("2"), input.get("3"), input.get("4")){

                                if *one && hand.len() >= 1{

                                    played_card = hand.remove(0);

                                } else if *two && hand.len() >= 2{

                                    played_card = hand.remove(1);

                                } else if *three && hand.len() >= 3{

                                    played_card = hand.remove(2);

                                } else if *four && hand.len() >= 4{

                                    played_card = hand.remove(3);

                                } else {

                                    return;

                                }

                            }

                        }

                    }

                }

            }

        }

    }

    if let (Some(value), Some(color)) = (played_card.get(VALUE), played_card.get(COLOR)){

        let output = format!("player{} has played {} of {}", active_player, value, color);
        println!("{}\r", output.bold());

    }

    if let Some(games) = entities.get_mut(GAMES){

        if let Some(game) = games.get_mut(0){

            if let Some(ecs::Components::V(game_stack)) = game.get_mut(DISCARD_PILE){

                temporary_stack = std::mem::take(game_stack);

            }

            if let Some(ecs::Components::I(active_player_temp)) = game.get_mut(ACTIVE_PLAYER){

                *active_player_temp = (*active_player_temp % 4) + 1;

            }

            if let Some(ecs::Components::B(new_turn)) = game.get_mut(NEW_TURN){

                *new_turn = true;

            }

        }

    }

    if let Some(top_card) = temporary_stack.last(){

        if let Some(ecs::Components::S(top_card_value)) = top_card.get(VALUE){

            if let Some(ecs::Components::S(played_card_value)) = played_card.get(VALUE){

                if played_card_value == top_card_value || *played_card_value == "Jack"{

                    if played_card_value == top_card_value{

                        println!("player{} played the same card as top card\r", active_player);

                    } else {

                        println!("player{} played a Jack\r", active_player);

                    }

                    if let Some(players) = entities.get_mut(PLAYERS){

                        for player in players{

                            if let Some(ecs::Components::I(number)) = player.get_mut(PLAYER_NUMBER){

                                if *number == active_player{

                                    if temporary_stack.len() == 1 && played_card_value == top_card_value{

                                        println!("player{} got extra points\r", active_player);

                                        if let Some(ecs::Components::I(points)) = player.get_mut(POINTS){

                                            if is_card_value_high_value(*played_card_value) {

                                                *points = *points + 20;
                                                println!("player{} scored 20 points\r", active_player);


                                            } else {

                                                *points = *points + 10;
                                                println!("player{} scored 10 points\r", active_player);

                                            }

                                        }

                                    }

                                    temporary_stack.push(played_card);

                                    println!("player{} took the discard pile\r", active_player);

                                    if let Some(ecs::Components::I(points)) = player.get_mut(POINTS){

                                        for card in &temporary_stack{

                                            if let Some(ecs::Components::S(value)) = card.get(VALUE){

                                                if is_card_value_high_value(*value) {

                                                    *points = *points + 1;
                                                    println!("player{} scored 1 point\r", active_player);

                                                }

                                            }

                                        }

                                        println!("\r");

                                    } else {

                                        println!("could not find component points in player{}", active_player);

                                    }

                                    if let Some(ecs::Components::V(stash)) = player.get_mut(STASH){

                                        stash.append(&mut temporary_stack);

                                    }

                                    if let Some(ecs::Components::V(stash)) = player.get(STASH){

                                        println!("player{}s stash is now:\r", active_player);

                                        for card in stash{

                                            if let (Some(value), Some(color)) = (card.get(VALUE), card.get(COLOR)){

                                                let output = format!("{} of {}", value, color);
                                                println!("{}\r", output.italic());

                                            }

                                        }

                                    } else {

                                        println!("error could not find player{}s stash", active_player);

                                    }

                                    if let Some(ecs::Components::I(points)) = player.get(POINTS){

                                        println!("player{}s total points are now {}\r", active_player, points);

                                    }

                                    return;

                                }

                            }

                        }

                    }

                }

            }

        }

    }

    temporary_stack.push(played_card);

    if let Some(games) = entities.get_mut(GAMES){

        if let Some(game) = games.get_mut(0){

            game.insert(DISCARD_PILE, ecs::Components::V(temporary_stack));

        }

    }

}

fn draw_cards_system(entities: &mut HashMap<&'static str, Vec<HashMap<&'static str, ecs::Components>>>, _input: &HashMap<&'static str, bool>){

    if all_hands_empty(entities){

        let mut drawn_cards: Vec<HashMap<&'static str, ecs::Components>> = vec![];
        let mut draw_pile_size = 0;

        if let Some(games) = entities.get_mut(GAMES){

            if let Some(game) = games.get_mut(0){

                if let Some(ecs::Components::V(draw_pile)) = game.get_mut(DRAW_PILE){

                    for _ in 0..(4*4){

                        if let Some(card) = draw_pile.pop(){

                            drawn_cards.push(card);

                        } else {

                            let output = format!("all cards have been played");
                            println!("{}\r", output.bold().red());
                            println!("\r");
                            return;

                        }

                    }

                    draw_pile_size = draw_pile.len();

                }

            }

        }

        let output = format!("all hands are empty, everyone will draw 4 cards");
        println!("{}\r", output.bold().yellow());

        if let Some(players) = entities.get_mut(PLAYERS){

            for player in players{

                if let Some(ecs::Components::V(hand)) = player.get_mut(HAND){

                    for _ in 0..4{

                        if let Some(card) = drawn_cards.pop(){

                            hand.push(card);

                        }

                    }

                }

            }

        }

        if let Some(players) = entities.get(PLAYERS){

            for player in players{

                println!("\r");

                if let Some(ecs::Components::V(hand)) = player.get(HAND){

                    for card in hand{

                        if let (Some(name), Some(value), Some(color)) = (player.get(NAME), card.get(VALUE), card.get(COLOR)){

                            let output = format!("{} has drawn {} of {}\r", name, value, color);
                            println!("{}", output.italic());

                        }

                    }

                }

            }

        }

        println!("\r");
        println!("draw pile now has {} cards left\r", draw_pile_size);
        println!("\r");

    }

}

fn exit_game(entities: &mut HashMap<&'static str, Vec<HashMap<&'static str, ecs::Components>>>){

    println!("\r");
    let output = format!("the game is now over");
    println!("{}\r", output.bold().red());
    println!("\r");

    if let Some(players) = entities.get(PLAYERS){

        for player in players{

            if let Some(number) = player.get(PLAYER_NUMBER){

                let output = format!("player{}s stash:", number);
                println!("{}\r", output.bold().blue())

            }

            if let Some(ecs::Components::V(stash)) = player.get(STASH){

                for card in stash{

                    if let (Some(value), Some(color)) = (card.get(VALUE), card.get(COLOR)){

                        println!("{} of {}\r", value, color);

                    }

                }

            }

            if let Some(ecs::Components::I(points)) = player.get(POINTS){

                let output = format!("total points: {}", points);
                println!("{}\r", output.bold().blue());
                println!("\r");

            }

        }

    }

    ecs::disable_input();
    exit(0);

}

fn exit_game_system(entities: &mut HashMap<&'static str, Vec<HashMap<&'static str, ecs::Components>>>, input: &HashMap<&'static str, bool>){

    let mut draw_pile_empty = false;

    if let Some(games) = entities.get(GAMES){

        if let Some(game) = games.get(0){

            if let Some(ecs::Components::V(draw_pile)) = game.get(DRAW_PILE){

                if draw_pile.len() <= 0{

                    draw_pile_empty = true;

                }

            }

        }

    }

    if draw_pile_empty && all_hands_empty(entities){

        exit_game(entities);

    }

    if let Some(back) = input.get("back"){

        if *back{

            exit_game(entities);

        }

    }

}

fn create_deck() -> Vec<HashMap<&'static str, ecs::Components>>{

    let mut cards = vec![];
    let types = vec![CLUBS, DIAMONDS, HEARTS, SPADES];
    let values = vec![TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE, TEN, JACK, QUEEN, KING, ACE];

    let mut counter = 0;

    for color in &types{

        for value in &values{

            let mut new_card = ecs::create_new_entity();
            new_card.insert("color", ecs::Components::S(color));
            new_card.insert("value", ecs::Components::S(value));
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
    return value == ACE || value == TEN || value == KING || value == QUEEN || value == JACK
}
