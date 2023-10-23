use std::fs::File;
use std::io::{self, BufRead};

use crate::structs::card::{Card, CardType}; // Importe a definição de Card e CardType

pub fn load_cards(filename: &str) -> Vec<Card> {
    let mut cards = Vec::new();

    if let Ok(file) = File::open(filename) {
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            if let Ok(card_data) = line {
                let parts: Vec<&str> = card_data.split(',').collect();
                if parts.len() >= 2 {
                    let name = parts[0].to_string();
                    let card_type = match parts[1] {
                        "Magic" => CardType::Magic,
                        "Monster" => {
                            if parts.len() >= 3 {
                                let atk = match parts[2].parse::<u8>() {
                                    Ok(value) => value,
                                    Err(_) => 0,
                                };
                                CardType::Monster(atk)
                            } else {
                                CardType::Monster(0)
                            }
                        }
                        _ => CardType::Magic,
                    };
                    let card = Card::new(name, card_type, None);
                    cards.push(card);
                }
            }
        }
    }

    cards
}
