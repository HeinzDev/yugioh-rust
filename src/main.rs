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
        
        //initialize the deck
        if deck.is_empty() {
            deck = load_cards("cards pt-br.txt");
        }

        //populate the hand with 5 cards
        if hand.is_empty() && phase_index == 1 {
            for _ in 0..5 {
                if let Some(drawn_card) = draw_random_card(&mut deck) {
                    hand.push(drawn_card);
                }
            }
        }

        //display hand
        println!("Essa é a sua mão:");
        display_hand(&hand);

        if phase_index == 2 || phase_index == 4 {
            let mut number:i32;
            let mut main_phase_input = String::new();
            loop{
                println!("Insira o número da carta para ver detalhes:");
                io::stdin().read_line(&mut main_phase_input).unwrap();

                if main_phase_input.trim().is_empty(){
                    break;
                }else{
                    number = to_int(&main_phase_input)-1;
                    card_details(&hand, number as usize);

                    main_phase_input = String::from("");
                }
            }
        }


        if interactive_phase(phase_index as i32) {
            println!("aperte enter para prosseguir");
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
