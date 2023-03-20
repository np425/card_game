#![allow(dead_code)]

use crate::card::minion::BodyBagger;
use crate::card::spell::HornOfWinter;
use crate::game::{Game, PlayerChoice};
use crate::player::Player;

mod card;
mod game;
mod player;
mod view;

fn main() {
    let player_a = Player::new(
        "Player A".to_string(),
        vec![
            Box::new(HornOfWinter::default()),
            Box::new(HornOfWinter::default()),
            Box::new(BodyBagger::default()),
        ],
        3,
        4,
        30,
    );

    let player_b = Player::new(
        "Player B".to_string(),
        vec![
            Box::new(HornOfWinter::default()),
            Box::new(HornOfWinter::default()),
            Box::new(BodyBagger::default()),
        ],
        2,
        4,
        30,
    );

    let mut game = Game::new([player_a, player_b], PlayerChoice::First);
    game.play();
}
