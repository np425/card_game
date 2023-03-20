use crate::card::{Card, Rarity};
use crate::game::Game;

pub trait Minion: Card {
    fn health(&self) -> u8;
    fn set_health(&mut self, health: u8);

    fn damage(&self) -> u8;
    fn set_damage(&mut self, damage: u8);
}

pub struct BodyBagger {
    mana: u8,
    health: u8,
    damage: u8,
}

impl Default for BodyBagger {
    fn default() -> Self {
        Self { mana: 1, health: 3, damage: 1 }
    }
}

impl Card for BodyBagger {
    fn name(&self) -> String {
        "Body Bagger".to_string()
    }

    fn desc(&self) -> String {
        "Battle-cry: Gain a Corpse".to_string()
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

    fn activate(&mut self, _game: &mut Game) {
        println!("A wild body badger appears!");
    }
}

impl Minion for BodyBagger {
    fn health(&self) -> u8 {
        self.health
    }

    fn set_health(&mut self, health: u8) {
        self.health = health;
    }

    fn damage(&self) -> u8 {
        self.damage
    }

    fn set_damage(&mut self, damage: u8) {
        self.damage = damage;
    }
}
