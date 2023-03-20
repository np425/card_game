use crate::card::Card;

#[derive(Default)]
pub struct Player {
    name: String,
    hand: Vec<Box<dyn Card>>,
    mana: u8,
    mana_available: u8,
    health: u8,
}

impl Player {
    pub fn new(
        name: String,
        hand: Vec<Box<dyn Card>>,
        mana: u8,
        mana_available: u8,
        health: u8,
    ) -> Self {
        Self {
            name,
            hand,
            mana,
            mana_available,
            health,
        }
    }

    pub fn hand(&self) -> &Vec<Box<dyn Card>> {
        &self.hand
    }

    pub fn take_card(&mut self, idx: usize) -> Option<Box<dyn Card>> {
        if idx >= self.hand.len() {
            None
        } else {
            Some(self.hand.remove(idx))
        }
    }

    pub fn mana(&self) -> u8 {
        self.mana
    }

    pub fn set_mana(&mut self, mana: u8) {
        self.mana = mana;
    }

    pub fn mana_available(&self) -> u8 {
        self.mana_available
    }

    pub fn set_mana_available(&mut self, mana: u8) {
        self.mana_available = mana
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn health(&self) -> u8 {
        self.health
    }
}
