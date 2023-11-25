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
    let mut magic_field: Vec<Option<Card>> = vec![None; 5];

    loop {
        println!("=====================================");
        println!("Fase Atual: {:?}", phases[phase_index]);
        let mut control = String::new();

        println!("");
        println!("botHP: {}", bot_hp);
        println!("{}",lifebar(bot_hp as u16));
        println!("");

        
        //initialize the deck
        if deck.is_empty() {
            deck = load_cards("cards.txt");
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
        display_hand(&hand);
    
        //main phases
        if phase_index == 2 || phase_index == 4 {
            let mut number:i32;
            let mut main_phase_input = String::new();
            let mut main_phase_detail_input = String::new();
            let mut move_done = false;
           loop {
                
                println!("Select an option");
                println!("1. Normal Summon a monster");
                println!("2. View the details of your cards");
                println!("3. View the Field");
                main_phase_input = String::from("");
                io::stdin().read_line(&mut main_phase_input).expect("Error: couldn't get the input");
            

                while main_phase_input.trim()=="1" {
                    println!("Your Field:");
                    print_field(&monster_field, &magic_field);
                    summon_monster(&mut hand, &mut monster_field);
                    move_done = true;
                    print_field(&monster_field, &magic_field);
                    break;
                }

                while main_phase_input.trim()=="2" {
                    println!("Insira o número da carta para ver detalhes:");
                    println!("[Digite nenhuma entrada para sair]");
                    io::stdin().read_line(&mut main_phase_detail_input).unwrap();

                    if main_phase_detail_input.trim().is_empty(){
                        break;
                    }else{
                        number = to_int(&main_phase_detail_input)-1;
                        card_details(&hand, number as usize);

                    main_phase_detail_input = String::from("");
                    }
                }

                if main_phase_input.trim()=="3" {
                    println!("Field");
                    print_field(&monster_field, &magic_field);
                    println!("");
                }

                if move_done {
                    break;
                }
            }
        }



        
        if phases[phase_index] == "Battle" {
            battle_phase(&mut hp, &mut bot_hp, &mut monster_field)
        }

        round += 1;
        phase_index = round % phases.len();
    }
}
