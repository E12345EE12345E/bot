#![allow(dead_code)]

use crate::constants::bot_constants::*;
use crate::constants::types::*;
use crate::game::Game;

pub trait Player {
    fn get_game(&self) -> &Game;
    fn get_game_mut(&mut self) -> &mut Game;
    fn get_next_move(&mut self) -> CommandList;

    fn make_move(&mut self) -> bool {
        if self.get_game().get_game_over() {
            return false;
        }
        let action = self.get_next_move();
        // println!("{:?}", action);
        do_move_list(self.get_game_mut(), action);
        true
    }

    fn make_n_moves(&mut self, n: usize) {
        for _ in 0..n {
            if !self.make_move() {
                break;
            }
        }
    }
}

pub fn do_move_list(game: &mut Game, commands: CommandList) {
    for command in commands {
        do_command(game, command);
    }
}

pub fn do_command(game: &mut Game, command: Command) -> bool {
    match command {
        Command::None => true,
        Command::MoveLeft => game.active_left(),
        Command::MoveRight => game.active_right(),
        Command::SoftDrop => game.active_drop(),
        Command::RotateCW => game.active_cw(),
        Command::RotateCCW => game.active_ccw(),
        Command::Rotate180 => game.active_180(),
        Command::Hold => {
            game.hold();
            true
        }
        Command::HardDrop => {
            let game_over = !game.hard_drop();
            game.set_game_over(game_over);
            true
        }
    }
}
