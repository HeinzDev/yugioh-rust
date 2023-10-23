use std::fs::File;
use std::io::{self, BufRead};

use crate::structs::card::{Card, CardType};

pub fn load_cards(filename: &str) -> Vec<Card> {
    let mut cards = Vec::new();

    if let Ok(file) = File::open(filename) {
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            if let Ok(card_data) = line {
                let parts: Vec<&str> = card_data.split(',').collect();
                if parts.len() >= 2 {
                    let name = parts[0].to_string();
                    let card = match parts[1] {
                        "Magic" => {
                            if parts.len() >= 3 {
                                let description = Some(parts[2].to_string());
                                Card::new(name, CardType::Magic, None, description)
                            } else {
                                Card::new(name, CardType::Magic, None, None)
                            }
                        }
                        "Monster" => {
                            if parts.len() >= 4 {
                                let atk = match parts[2].parse::<u16>() {
                                    Ok(value) => Some(value),
                                    Err(_) => None,
                                };
                                let description = Some(parts[3].to_string()); // Lê a descrição
                                Card::new(name, CardType::Monster(atk), atk, description)
                            } else {
                                Card::new(name, CardType::Monster(None), None, None)
                            }
                        }
                        _ => {
                            Card::new(name, CardType::Magic, None, None)
                        }
                    };
                    cards.push(card);
                }
            }
        }
    }

    cards
}


pub fn hand_details(deck: &Vec<Card>) {
    for (index, card) in deck.iter().enumerate() {
        match &card.card_type {
            CardType::Magic => {
                println!("Card {}: Name: {}, Type: Magic", index + 1, card.name);
            }
            CardType::Monster(atk) => {
                match atk {
                    Some(value) => {
                        println!(
                            "Card {}: Name: {}, Type: Monster, Attack: {}",
                            index + 1,
                            card.name,
                            value
                        );
                    }
                    None => {
                        println!("Card {}: Name: {}, Type: Monster, Attack: None", index + 1, card.name);
                    }
                }
            }
        }
    }
}

pub fn card_details(hand: &Vec<Card>, index: usize) {
    if let Some(card) = hand.get(index) {
        println!("{:?}", card.card_type);
        match &card.card_type {
            CardType::Magic => {
                println!("Name: {}, Type: Magic, Description:{:?}", card.name, card.description);
            }
            CardType::Monster(atk) => {
                match atk {
                    Some(value) => {
                        println!("Name: {}, Type: Monster, Attack: {}, Description:{:?}", card.name, value, card.description);
                    }
                    None => {
                        println!("Name: {}, Type: Monster, Attack: None, Description:{:?}", card.name, card.description);
                    }
                }
            }
        }
    } else {
        println!("Card not found at index {}", index);
    }
}
