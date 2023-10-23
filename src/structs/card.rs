#[derive(Clone, Debug)]
pub enum CardType {
    Magic,
    Monster(Option<u16>), // O valor dentro do parêntese representa o ataque do monstro
}
#[derive(Clone, Debug)]
pub struct Card {
    pub name: String,
    pub card_type: CardType,
    pub atk: Option<u16>,
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
