use crate::structs::card::Card;
use rand::seq::SliceRandom;

use std::io;

pub fn dead(i: i32, j: i32) -> bool {
    i <= 0 || j <= 0
}

pub fn to_int(input: &str) -> i32 {
    return input.trim().parse::<i32>().unwrap();
}

pub fn attack(bot_hp: i32) -> i32 {
    let mut input = String::new();
    println!("Digite o seu valor de ataque!");
    io::stdin().read_line(&mut input).ok();

    let input = input.trim();

    if input.is_empty() {
        return 0;
    }

    println!("input: {}", input);
    return bot_hp - to_int(&input);
}

pub fn interactive_phase(i: i32) -> bool {
    match i {
        2 | 3 | 4 => return true,
        _ => return false,
    }
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
