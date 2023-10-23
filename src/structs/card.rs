#[derive(Clone)]
pub enum CardType {
    Magic,
    Monster(u8), // O valor dentro do parêntese representa o ataque do monstro
}
#[derive(Clone)]
pub struct Card {
    name: String,
    card_type: CardType,
    atk: Option<u16>,
}

impl Card {
    pub fn new(name: String, card_type: CardType, atk: Option<u16>) -> Self {
        Card {
            name,
            card_type,
            atk,
        }
    }
}
