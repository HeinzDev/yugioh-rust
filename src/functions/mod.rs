use crate::structs::card::Card;
use rand::Rng;
//use rand::seq::SliceRandom;
use std::io;

pub fn dead(i: i32, j: i32) -> bool {
    i <= 0 || j <= 0
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


pub fn draw_random_card(deck: &mut Vec<Card>) -> Option<Card> {
    if deck.is_empty() {
        return None;
    }

    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..deck.len());

    Some(deck.remove(random_index))
}



pub fn display_hand(hand: &Vec<Card>) {
    print!("Mão: [");
    for (index, card) in hand.iter().enumerate() {
        if index > 0 {
            print!(", ");
        }
        print!("'{}'", &card.name);
    }
    println!("]");
}


pub fn attack(bot_hp: i32) -> i32 {
    let mut input = String::new();
    println!("Digite o seu valor de ataque!");
    io::stdin().read_line(&mut input).ok();

    let input = input.trim();

    if input.is_empty() {
        return bot_hp;
    }
    return bot_hp - to_int(&input);
}

pub fn interactive_phase(i: i32) -> bool {
    match i {
        2 | 3 | 4 => return true,
        _ => return false,
    }
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

pub fn find_empty_monster_slot(field: &Vec<Option<Card>>) -> Option<usize> {
    for (index, slot) in field.iter().enumerate() {
        if slot.is_none() {
            return Some(index);
        }
    }
    None
}

pub fn summon_monster(field: &mut Vec<Option<Card>>, card: Card, slot_index: usize) {
    if let Some(slot) = field.get_mut(slot_index) {
        *slot = Some(card);
    }
}

pub fn kill_monster(field: &mut Vec<Option<Card>>, slot_index: usize) -> Option<Card> {
    if let Some(slot) = field.get_mut(slot_index) {
        slot.take()
    } else {
        None
    }
}
