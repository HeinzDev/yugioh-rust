#[derive(Clone, Debug)]
pub enum CardType {
    Magic,
    Monster(Option<u16>), // O valor dentro do parêntese representa o ataque do monstro
}
#[derive(Clone)]
#[derive(Debug)]
pub struct Card {
    pub name: String,
    pub card_type: CardType,
    pub atk: Option<u16>,
    pub description: Option<String>, // Novo campo para a descrição
}


impl Card {
    pub fn new(name: String, card_type: CardType, atk: Option<u16>, description: Option<String>,) -> Self {
        Card {
            name,
            card_type,
            atk,
            description
        }
    }
}
