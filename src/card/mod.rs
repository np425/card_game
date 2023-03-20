use crate::game::Game;

pub mod spell;
pub mod minion;

pub enum Rarity {
    Neutral,
    Common,
    Rare,
    Epic,
    Legendary,
}

pub trait Card {
    fn name(&self) -> String;
    fn desc(&self) -> String;

    fn mana(&self) -> u8;
    fn set_mana(&mut self, mana: u8);

    fn rarity(&self) -> Rarity;

    fn activate(&mut self, game: &mut Game);
}

