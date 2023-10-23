mod functions;
mod loadCards;
mod structs;

use functions::*;
use loadCards::*;
use std::io;
use structs::card::Card;

fn main() {
    let mut bot_hp: i32 = 8000;
    let mut phase_index = 0;
    let mut round = 0;
    let mut deck = vec![];

    let phases = vec!["Standby", "Draw", "Main", "Battle", "MainII", "End"];
    let mut hand: Vec<Card> = vec![];
    let mut monster_field: Vec<Option<Card>> = vec![None; 5];

    loop {
        let mut control = String::new();
        if hand.is_empty() {
            deck = load_cards("cards.txt");
        }
        if interactive_phase(phase_index as i32) {
            io::stdin().read_line(&mut control).unwrap();
        }

        if phases[phase_index] == "Battle" {
            bot_hp = attack(bot_hp);
            if dead(10, bot_hp) {
                break;
            }
        }

        println!("botHP: {}", bot_hp);
        round += 1;
        phase_index = round % phases.len();
        println!("Fase Atual: {:?}", phases[phase_index]);
    }
}
