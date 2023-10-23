use std::fs::File;
use std::io::{self, BufRead};

use crate::structs::card::{Card, CardType};

pub fn load_cards(filename: &str) -> Vec<Card> {
    let mut cards = Vec::new();

    match File::open(filename) {
        Ok(file) => {
            let reader = io::BufReader::new(file);

            for line in reader.lines() {
                if let Ok(card_data) = line {
                    println!("Linha lida: {}", card_data);
                    let parts: Vec<&str> = card_data.split(',').collect();
                    if parts.len() >= 2 {
                        let name = parts[0].to_string();
                        let card_type = match parts[1] {
                            "Magic" => CardType::Magic,
                            "Monster" => {
                                if parts.len() >= 3 {
                                    let atk = match parts[2].parse::<u16>() {
                                        Ok(value) => Some(value),
                                        Err(_) => None,
                                    };
                                    CardType::Monster(atk)
                                } else {
                                    CardType::Monster(None)
                                }
                            }
                            _ => CardType::Magic,
                        };
                        let card = Card::new(name, card_type, None);
                        cards.push(card.clone());
                        //println!("Carta adicionada: {:?}", card);
                    }
                }
            }
        }
        Err(e) => println!("Erro ao abrir o arquivo: {}", e),
    }

    //println!("Número de cartas carregadas: {}", cards.len());

    cards
}

pub fn display_deck(deck: &Vec<Card>) {
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

