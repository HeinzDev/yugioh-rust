mod functions;
mod loadCards;
mod structs;

use functions::*;
use loadCards::*;
use std::io;
use structs::card::Card;

fn main() {
    let mut bot_hp: i32 = 8000;
    let mut hp: i32 = 8000;
    let mut phase_index = 0;
    let mut round = 0;
    let mut deck = vec![];
    
    let phases = vec!["Standby", "Draw", "Main", "Battle", "MainII", "End"];
    let mut hand: Vec<Card> = vec![];
    let mut monster_field: Vec<Option<Card>> = vec![None; 5];
    
    loop {
        println!("Fase Atual: {:?}", phases[phase_index]);
        let mut control = String::new();

        if deck.is_empty() {
            deck = load_cards("cards.txt");
        }

        if hand.is_empty() && phase_index == 1 {
            for _ in 0..5 {
                if let Some(drawn_card) = draw_random_card(&mut deck) {
                    println!("ADICIONOU CARTA");
                    hand.push(drawn_card);
                }
            }
        }

        display_hand(&hand);

        if interactive_phase(phase_index as i32) {
            io::stdin().read_line(&mut control).unwrap();
        }
        println!("botHP: {}", bot_hp);
        println!("{}",lifebar(bot_hp as u16));

        if phases[phase_index] == "Battle" {
            bot_hp = attack(bot_hp);
            if dead(10, bot_hp) {
                break;
            }
        }

        round += 1;
        phase_index = round % phases.len();
    }
}
