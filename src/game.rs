use crate::card::Card;
use crate::player::Player;
use crate::view::View;

#[derive(Default)]
pub enum PlayerChoice {
    #[default]
    First,
    Second,
}

impl PlayerChoice {
    fn next(&self) -> Self {
        match self {
            Self::First => Self::Second,
            Self::Second => Self::First,
        }
    }
}

#[derive(Default)]
pub struct Game {
    view: View,
    players: [Player; 2],
    to_play: PlayerChoice,
}

impl Game {
    pub fn new(players: [Player; 2], to_play: PlayerChoice) -> Self {
        Self {
            view: View::default(),
            players,
            to_play,
        }
    }

    pub fn play(&mut self) {
        loop {
            self.view.display_game_info(self);
            let card_idx = self.view.request_card_in_hand(self);
            println!("Index {card_idx}");
            self.play_card(card_idx);
            //self.next_turn();
        }
    }

    fn next_turn(&mut self) {
        let pl = self.cur_pl_mut();

        pl.set_mana_available(pl.mana_available() + 1);
        pl.set_mana(pl.mana_available());

        self.to_play = self.to_play.next();
    }

    pub fn play_card(&mut self, idx: usize) -> Option<Box<dyn Card>> {
        let pl = self.cur_pl_mut();
        let card = pl.hand().get(idx)?;

        if card.mana() > pl.mana() {
            println!("Not enough mana!");
            return None;
        }

        let mut card = pl.take_card(idx)?;
        pl.set_mana(pl.mana() - card.mana());
        card.activate(self);
        Some(card)
    }

    pub fn cur_pl(&self) -> &Player {
        match self.to_play {
            PlayerChoice::First => &self.players[0],
            PlayerChoice::Second => &self.players[1],
        }
    }

    pub fn cur_pl_mut(&mut self) -> &mut Player {
        match self.to_play {
            PlayerChoice::First => &mut self.players[0],
            PlayerChoice::Second => &mut self.players[1],
        }
    }

    pub fn enemy(&self) -> &Player {
        match self.to_play {
            PlayerChoice::First => &self.players[1],
            PlayerChoice::Second => &self.players[0],
        }
    }

    pub fn enemy_mut(&mut self) -> &mut Player {
        match self.to_play {
            PlayerChoice::First => &mut self.players[1],
            PlayerChoice::Second => &mut self.players[0],
        }
    }
}
