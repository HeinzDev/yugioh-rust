use crate::structs::card::Card;
use crate::structs::card::CardType;
use rand::Rng;
//use rand::seq::SliceRandom;
use std::io;


pub fn dead(i: &i32, j: &i32) -> bool {
    i <= &0 || j <= &0
}

pub fn to_int(input: &str) -> i32 {
    return input.trim().parse::<i32>().unwrap();
}

pub fn draw_card(deck: &mut Vec<Card>) -> Option<Card> {
    if let Some(card) = deck.pop() {
        Some(card)
    } else {
        None
    }
}

pub fn buy_phase(deck: &mut Vec<Card>, hand: &mut Vec<Card>) {
    // Comprar uma nova carta e adicioná-la à mão
    if let Some(new_card) = draw_random_card(deck) {
        hand.push(new_card.clone());
        println!("You bought a new card: {}", new_card.name);
    } else {
        println!("No cards left in the deck.");
    }
}

pub fn draw_random_card(deck: &mut Vec<Card>) -> Option<Card> {
    if deck.is_empty() {
        return None;
    }

    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..deck.len());

    Some(deck.remove(random_index))
}


pub fn display_hand(hand: &Vec<Card>) {
    println!("Hand Cards:"); 
    println!("");
    print!("[");
    for (index, card) in hand.iter().enumerate() {
        if index > 0 {
            print!(", ");
        }
        print!("'{}'", &card.name);
    }
    println!("]");
    println!("");
}

pub fn lifebar(mut life:u16)-> String{
    let mut lifebar = String::from('[');
    life = life /1000;
    let index = life % 10;
    for _ in 0..index{
        lifebar.push('=');
    }
    lifebar.push(']');
    return lifebar;
}

pub fn print_field<T>(monster_field: &[Option<T>], magic_field: &[Option<T>]) {
    println!("");
    for i in 0..monster_field.len() {
        if let Some(_) = &monster_field[i]{
            print!("[█]");
        } else {
            print!("[]");
        }    
    }
    
    println!("");

    for i in 0..magic_field.len() {
        if let Some(_) = &magic_field[i]{
            print!("(█)");
        } else {
            print!("()");
        }    
    }
    println!("");
}

pub fn find_empty_monster_slot(field: &Vec<Option<Card>>) -> Option<usize> {
    for (index, slot) in field.iter().enumerate() {
        if slot.is_none() {
            return Some(index);
        }
    }
    None
}


pub fn summon_monster(hand: &mut Vec<Card>, monster_field: &mut Vec<Option<Card>>, magic_field: &mut Vec<Option<Card>>) {
    display_hand(&hand);

    // Solicitar ao usuário que escolha uma carta de monstro para invocar
    println!("Choose a number from 1 - {} to summon a monster from your hand or 0 to cancel:", hand.len());
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erro ao obter o input");

    let choice = input.trim().parse::<usize>();

    match choice {
        Ok(index) if index > 0 && index <= hand.len() => {
            let selected_monster = &hand[index - 1];

            if let CardType::Monster(_) = selected_monster.card_type {
                println!("Choose the monster position [1 to 5]");
                let mut field_position = String::new();
                io::stdin().read_line(&mut field_position).expect("Error: couldn't get the input");

                let position = field_position.trim().parse::<usize>();

                match position {
                    Ok(pos) if pos > 0 && pos <= monster_field.len() => {
                        let summoned_monster = hand.remove(index - 1);

                        monster_field[pos - 1] = Some(summoned_monster);
                        println!("Monster Summoned!");
                    }
                    _ => println!("Invalid position. Summon canceled."),
                }
            } else {
                println!("Invalid choice. You can only summon Monster cards to the Monster field.");
            }
        }
        Ok(0) => println!("Summon canceled."),
        _ => println!("Invalid option."),
    }
}

pub fn kill_monster(field: &mut Vec<Option<Card>>, slot_index: usize) -> Option<Card> {
    if let Some(slot) = field.get_mut(slot_index) {
        slot.take()
    } else {
        None
    }
}



pub fn summon_magic(hand: &mut Vec<Card>, magic_field: &mut Vec<Option<Card>>) {
    display_hand(&hand);

    // Solicitar ao usuário que escolha uma carta de magia para invocar
    println!("Choose a number from 1 - {} to summon a magic card from your hand or 0 to cancel:", hand.len());
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error: couldn't get the input");

    let choice = input.trim().parse::<usize>();

    match choice {
        Ok(index) if index > 0 && index <= hand.len() => {
            let selected_magic = &hand[index - 1];

            if let CardType::Magic = selected_magic.card_type {
                println!("Choose the magic position [1 to 5]");
                let mut field_position = String::new();
                io::stdin().read_line(&mut field_position).expect("Error: couldn't get the input");

                let position = field_position.trim().parse::<usize>();

                match position {
                    Ok(pos) if pos > 0 && pos <= magic_field.len() => {
                        let summoned_magic = hand.remove(index - 1);

                        magic_field[pos - 1] = Some(summoned_magic);
                        println!("Magic Card Summoned!");
                    }
                    _ => println!("Invalid position. Summon canceled."),
                }
            } else {
                println!("Invalid choice. You can only summon Magic cards to the Magic field.");
            }
        }
        Ok(0) => println!("Summon canceled."),
        _ => println!("Invalid option."),
    }
}


pub fn battle_phase(bot_hp: &mut i32, monster_field: &mut Vec<Option<Card>>, magic_field: &mut Vec<Option<Card>>) {
    println!("Your Field:");
    print_field(monster_field, magic_field);

    println!("Select a monster position to attack (1 to 5) or 0 to cancel:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error: couldn't get the input");

    let choice = input.trim().parse::<usize>();

    match choice {
        Ok(index) if index > 0 && index <= monster_field.len() => {
            if let Some(attacking_monster) = &monster_field[index - 1] {
                if let CardType::Monster(atk) = &attacking_monster.card_type {
                    if let Some(damage) = *atk {
                        *bot_hp -= damage as i32;

                        println!("Your monster attacked successfully! Dealt {} damage to the opponent.", damage);
                    } else {
                        println!("The selected monster has no attack value.");
                    }
                } else {
                    println!("The selected card is not a monster.");
                }
            } else {
                println!("There's no monster in that position.");
            }
        }
        Ok(0) => println!("Attack canceled."),
        _ => println!("Invalid option."),
    }
}
