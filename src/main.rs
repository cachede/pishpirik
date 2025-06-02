mod ecs;
use std::collections::HashMap;
use std::process::exit;

use rand::seq::SliceRandom;
use rand::thread_rng;
use colored::Colorize;

type Entities = HashMap<&'static str, Vec<HashMap<&'static str, ecs::Components>>>;
type Cards = Vec<HashMap<&'static str, ecs::Components>>;

const DISCARD_PILE_MIN_CAPACITY: usize = 4;

fn fill_discard_pile(entities: &mut Entities, cards: &mut Cards) -> Option<()> {
    let games = entities.get_mut("games")?;
    let game = games.get_mut(0)?;
    let mut discard_pile = match game.get_mut("discard pile") {
        Some(ecs::Components::V(vec)) => vec,
        _ => return None
    };

    let mut index = 0;

    while discard_pile.len() < DISCARD_PILE_MIN_CAPACITY {

        let card1 = cards.get_mut(index)?;
        let mut is_not_jack = false;

        if let (ecs::Components::S(pic), ecs::Components::S(color)) =
            (card1.get("value")?, card1.get("color")?)
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

    let games = entities.get_mut("games")?;
    let game = games.get_mut(0)?;
    let mut draw_pile = match game.get_mut("draw pile") {
        Some(ecs::Components::V(vec)) => vec,
        _ => return None
    };

    draw_pile.append(cards);

    println!("\r");
    println!("draw pile is now {} big", draw_pile.len());
    println!("\r");

    Some(())
}

fn main() {

    let mut entities = ecs::new_entities_repo();
    let mut systems = ecs::new_systems_repo();

    let mut player1 = ecs::create_new_entity();
    player1.insert("name", ecs::Components::S("player1"));
    player1.insert("number", ecs::Components::I(1));
    player1.insert("hand", ecs::Components::V(vec![]));
    player1.insert("stash", ecs::Components::V(vec![]));
    player1.insert("points", ecs::Components::I(0));
    ecs::add_entity_to_group(&mut entities, player1, "players");

    let mut player2 = ecs::create_new_entity();
    player2.insert("name", ecs::Components::S("player2"));
    player2.insert("number", ecs::Components::I(2));
    player2.insert("hand", ecs::Components::V(vec![]));
    player2.insert("stash", ecs::Components::V(vec![]));
    player2.insert("points", ecs::Components::I(0));
    ecs::add_entity_to_group(&mut entities, player2, "players");

    let mut player3 = ecs::create_new_entity();
    player3.insert("name", ecs::Components::S("player3"));
    player3.insert("number", ecs::Components::I(3));
    player3.insert("hand", ecs::Components::V(vec![]));
    player3.insert("stash", ecs::Components::V(vec![]));
    player3.insert("points", ecs::Components::I(0));
    ecs::add_entity_to_group(&mut entities, player3, "players");

    let mut player4 = ecs::create_new_entity();
    player4.insert("name", ecs::Components::S("player4"));
    player4.insert("number", ecs::Components::I(4));
    player4.insert("hand", ecs::Components::V(vec![]));
    player4.insert("stash", ecs::Components::V(vec![]));
    player4.insert("points", ecs::Components::I(0));
    ecs::add_entity_to_group(&mut entities, player4, "players");

    let mut game = ecs::create_new_entity();
    game.insert("active_player", ecs::Components::I(1));
    game.insert("discard pile", ecs::Components::V(vec![]));
    game.insert("draw pile", ecs::Components::V(vec![]));
    game.insert("new turn", ecs::Components::B(true));
    ecs::add_entity_to_group(&mut entities, game, "games");

    let mut cards: Cards = create_deck();

    cards.shuffle(&mut thread_rng());

    fill_discard_pile(&mut entities, &mut cards);

    fill_draw_pile(&mut entities, &mut cards);

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

fn new_turn_system(entities: &mut HashMap<&'static str, Vec<HashMap<&'static str, ecs::Components>>>, _input: &HashMap<&'static str, bool>){

    let mut all_hands_empty = true;

    if let Some(players) = entities.get("players"){

        for player in players{

            if let Some(ecs::Components::V(hand)) = player.get("hand"){

                if !hand.is_empty(){

                    all_hands_empty = false

                }

            }

        }

    }

    if all_hands_empty{

        return;

    }

    if let Some(games) = entities.get_mut("games"){

        if let Some(game) = games.get_mut(0){

            if let Some(ecs::Components::B(new_turn)) = game.get_mut("new turn"){

                if !*new_turn{

                    return;

                } else {

                    *new_turn = false;

                }

            }

        }

    }

    let mut active_player: i32 = 1;

    if let Some(games) = entities.get("games"){

        if let Some(game) = games.get(0){

            if let Some(ecs::Components::I(active_player_temp)) = game.get("active_player"){

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

    if let Some(players) = entities.get("players"){

        for player in players{

            if let Some(ecs::Components::I(number)) = player.get("number"){

                if *number == active_player{

                    if let Some(ecs::Components::V(hand)) = player.get("hand"){

                        for (index, card) in hand.iter().enumerate() {

                            if let (Some(value), Some(color)) = (card.get("value"), card.get("color")) {

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

    if let Some(games) = entities.get("games"){

        if let Some(game) = games.get(0){

            if let Some(ecs::Components::V(discard_pile)) = game.get("discard pile"){

                if let Some(top_card) = discard_pile.last(){

                    if let (Some(value), Some(color)) = (top_card.get("value"), top_card.get("color")){

                        let output = format!("top card is {} of {}", value, color);
                        println!("{}\r", output.bold().blue());
                        println!("\r");

                    }

                }

            }

        }

    }

}

fn play_cards_system(entities: &mut HashMap<&'static str, Vec<HashMap<&'static str, ecs::Components>>>, input: &HashMap<&'static str, bool>){

    let mut active_player: i32 = 1;
    let mut temporary_stack: Vec<HashMap<&'static str, ecs::Components>> = vec![];
    let mut played_card: HashMap<&'static str, ecs::Components> = HashMap::new();

    if let Some(games) = entities.get("games"){

        if let Some(game) = games.get(0){

            if let Some(ecs::Components::I(active_player_temp)) = game.get("active_player"){

                active_player = *active_player_temp;

            }

        }

    }

    if let Some(players) = entities.get_mut("players"){

        for player in players{

            if let Some(ecs::Components::I(number)) = player.get("number"){

                if *number == active_player{

                    if let Some(ecs::Components::V(hand)) = player.get_mut("hand"){

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

    if let (Some(value), Some(color)) = (played_card.get("value"), played_card.get("color")){

        let output = format!("player{} has played {} of {}", active_player, value, color);
        println!("{}\r", output.bold());

    }

    if let Some(games) = entities.get_mut("games"){

        if let Some(game) = games.get_mut(0){

            if let Some(ecs::Components::V(game_stack)) = game.get_mut("discard pile"){

                temporary_stack = std::mem::take(game_stack);

            }

            if let Some(ecs::Components::I(active_player_temp)) = game.get_mut("active_player"){

                *active_player_temp = (*active_player_temp + 1) % 5;

                if *active_player_temp == 0{

                    *active_player_temp = 1;

                }

            }

            if let Some(ecs::Components::B(new_turn)) = game.get_mut("new turn"){

                *new_turn = true;

            }

        }

    }

    if let Some(top_card) = temporary_stack.last(){

        if let Some(ecs::Components::S(top_card_value)) = top_card.get("value"){

            if let Some(ecs::Components::S(played_card_value)) = played_card.get("value"){

                if played_card_value == top_card_value || *played_card_value == "Jack"{

                    if played_card_value == top_card_value{

                        println!("player{} played the same card as top card\r", active_player);

                    } else {

                        println!("player{} played a Jack\r", active_player);

                    }

                    if let Some(players) = entities.get_mut("players"){

                        for player in players{

                            if let Some(ecs::Components::I(number)) = player.get_mut("number"){

                                if *number == active_player{

                                    if temporary_stack.len() == 1 && played_card_value == top_card_value{

                                        println!("player{} got extra points\r", active_player);

                                        if let Some(ecs::Components::I(points)) = player.get_mut("points"){

                                            if *played_card_value == "Ace" || *played_card_value == "10" || *played_card_value == "King" || *played_card_value == "Queen" || *played_card_value == "Jack" {

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

                                    if let Some(ecs::Components::I(points)) = player.get_mut("points"){

                                        for card in &temporary_stack{

                                            if let Some(ecs::Components::S(value)) = card.get("value"){

                                                if *value == "Ace" || *value == "10" || *value == "King" || *value == "Queen" || *value == "Jack" {

                                                    *points = *points + 1;
                                                    println!("player{} scored 1 point\r", active_player);

                                                }

                                            }

                                        }

                                        println!("\r");

                                    } else {

                                        println!("could not find component points in player{}", active_player);

                                    }

                                    if let Some(ecs::Components::V(stash)) = player.get_mut("stash"){

                                        stash.append(&mut temporary_stack);

                                    }

                                    if let Some(ecs::Components::V(stash)) = player.get("stash"){

                                        println!("player{}s stash is now:\r", active_player);

                                        for card in stash{

                                            if let (Some(value), Some(color)) = (card.get("value"), card.get("color")){

                                                let output = format!("{} of {}", value, color);
                                                println!("{}\r", output.italic());

                                            }

                                        }

                                    } else {

                                        println!("error could not find player{}s stash", active_player);

                                    }

                                    if let Some(ecs::Components::I(points)) = player.get("points"){

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

    if let Some(games) = entities.get_mut("games"){

        if let Some(game) = games.get_mut(0){

            game.insert("discard pile", ecs::Components::V(temporary_stack));

        }

    }

}

fn draw_cards_system(entities: &mut HashMap<&'static str, Vec<HashMap<&'static str, ecs::Components>>>, _input: &HashMap<&'static str, bool>){

    let mut all_hands_empty = true;

    if let Some(players) = entities.get("players"){

        for player in players{

            if let Some(ecs::Components::V(hand)) = player.get("hand"){

                if !hand.is_empty(){

                    all_hands_empty = false

                }

            }

        }

    }

    if all_hands_empty{

        let mut drawn_cards: Vec<HashMap<&'static str, ecs::Components>> = vec![];
        let mut draw_pile_size = 0;

        if let Some(games) = entities.get_mut("games"){

            if let Some(game) = games.get_mut(0){

                if let Some(ecs::Components::V(draw_pile)) = game.get_mut("draw pile"){

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

        if let Some(players) = entities.get_mut("players"){

            for player in players{

                if let Some(ecs::Components::V(hand)) = player.get_mut("hand"){

                    for _ in 0..4{

                        if let Some(card) = drawn_cards.pop(){

                            hand.push(card);

                        }

                    }

                }

            }

        }

        if let Some(players) = entities.get("players"){

            for player in players{

                println!("\r");

                if let Some(ecs::Components::V(hand)) = player.get("hand"){

                    for card in hand{

                        if let (Some(name), Some(value), Some(color)) = (player.get("name"), card.get("value"), card.get("color")){

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

    if let Some(players) = entities.get("players"){

        for player in players{

            if let Some(number) = player.get("number"){

                let output = format!("player{}s stash:", number);
                println!("{}\r", output.bold().blue())

            }

            if let Some(ecs::Components::V(stash)) = player.get("stash"){

                for card in stash{

                    if let (Some(value), Some(color)) = (card.get("value"), card.get("color")){

                        println!("{} of {}\r", value, color);

                    }

                }

            }

            if let Some(ecs::Components::I(points)) = player.get("points"){

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
    let mut all_hands_empty = true;

    if let Some(players) = entities.get("players"){

        for player in players{

            if let Some(ecs::Components::V(hand)) = player.get("hand"){

                if !hand.is_empty(){

                    all_hands_empty = false

                }

            }

        }

    }

    if let Some(games) = entities.get("games"){

        if let Some(game) = games.get(0){

            if let Some(ecs::Components::V(draw_pile)) = game.get("draw pile"){

                if draw_pile.len() <= 0{

                    draw_pile_empty = true;

                }

            }

        }

    }

    if draw_pile_empty && all_hands_empty{

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
    let types = vec!["clubs", "diamonds", "hearts", "spades"];
    let values = vec!["2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King", "Ace"];

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
