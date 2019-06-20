use hdk::holochain_core_types::{
    error::HolochainError,
    json::JsonString,
};
use hdk::AGENT_ADDRESS;

use crate::game_move::Move;
use crate::game::Game;
use super::{
    moves::Piece,
    MoveType,
    validation::{Player, get_current_player},
};

pub const BOARD_SIZE: usize = 3;
pub const PLAYER_1_MARK: char = 'O';
pub const PLAYER_2_MARK: char = 'X';  //player 2 / Xs go first
pub const EMPTY_SPACE: char = ' ';

/**
 *
 * As a game author you get to decide what the State object of your game looks like.
 * Most of the time you want it to include all of the previous moves as well.
 * 
 * To customize the game state implement your own GameState struct. This must have a function called `initial()`
 * which returns the initial state.
 *
 */


#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct GameState {
    // pub moves: Vec<Move>,
    // Implement your own game state
    // May be helpful to split this into state for each player
}


impl GameState {
    pub fn initial() -> Self {
        // return an initial state of a game
    }

    pub fn render(&self) -> String {
        // return a pretty formatting string representation
    }

    pub fn evolve(&self, game: Game, next_move: &Move) -> GameState {
        // given a current state, a game and a move, compute the next state
        // You can assume all moves are valid
    }

}
