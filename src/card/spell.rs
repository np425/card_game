use crate::card::{Card, Rarity};
use crate::game::Game;

pub trait Spell: Card {}

pub struct HornOfWinter {
    mana: u8,
    refresh_amount: u8,
}

impl Default for HornOfWinter {
    fn default() -> Self {
        Self {
            mana: 0,
            refresh_amount: 2,
        }
    }
}

impl Card for HornOfWinter {
    fn name(&self) -> String {
        "Horn of Winter".to_string()
    }

    fn desc(&self) -> String {
        "Refresh 2 mana crystals".to_string()
    }

    fn mana(&self) -> u8 {
        self.mana
    }

    fn set_mana(&mut self, mana: u8) {
        self.mana = mana;
    }

    fn rarity(&self) -> Rarity {
        Rarity::Common
    }

    fn activate(&mut self, game: &mut Game) {
        let pl = game.cur_pl_mut();
        pl.set_mana(pl.mana() + self.refresh_amount);
    }
}

impl Spell for HornOfWinter {}
