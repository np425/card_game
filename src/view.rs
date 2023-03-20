use std::io;
use std::io::{stdin, Write};

use crate::card::Card;
use crate::game::Game;
use crate::player::Player;

#[derive(Default)]
pub struct View;

impl View {
    pub fn display_player_info(&self, player: &Player) {
        println!("/----------------------\\");
        println!("| Player: {:12} |", player.name());
        println!(
            "| Mana: {:14} |",
            format!("{} / {}", player.mana(), player.mana_available())
        );
        println!("| Health: {:12} |", player.health().to_string());
    }

    pub fn display_game_info(&self, game: &Game) {
        self.display_player_info(game.enemy());
        println!();
        self.display_player_info(game.cur_pl());

        println!("--- Hand: ---");
        self.display_cards(game.cur_pl().hand());
    }

    // TODO: Allow ending turn
    pub fn request_card_in_hand(&self, _game: &Game) -> usize {
        let mut line = String::new();
        let stdin = stdin();
        loop {
            print!("Enter card number: ");
            io::stdout().flush().unwrap();

            if let Err(_) = stdin.read_line(&mut line) {
                println!("Try again!");
                continue;
            }

            if let Ok(number) = line.trim().parse() {
                return number;
            }

            println!("Try again!");
        }
    }

    pub fn display_cards(&self, cards: &Vec<Box<dyn Card>>) {
        let card_length: usize = 30;

        for _ in cards {
            print!("_/{:-<card_length$}\\_ ", "");
        }
        println!();

        for card in cards {
            print!(
                "| {mana:4} {name:width$} | ",
                width = card_length - 5,
                name = card.name(),
                mana = format!("[{}]", card.mana())
            );
        }
        println!();

        for card in cards {
            let name_length: usize = card.name().len();
            print!(
                "| {:-<width$} {:width2$}| ",
                "-",
                "",
                width = name_length + 6,
                width2 = card_length - name_length - 6
            );
        }
        println!();

        for card in cards {
            print!("| {:card_length$} | ", card.desc());
        }
        println!();

        for _ in cards {
            print!("^/{:-<card_length$}\\^ ", "");
        }
        println!();
    }
}
