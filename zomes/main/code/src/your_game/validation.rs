use hdk::holochain_core_types::{
    cas::content::Address,
};

use crate::game::Game;
use crate::game_move::Move;
use super::{
    GameState,
    moves::Piece,
    MoveType,
    state::{board_sparse_to_dense, BOARD_SIZE},
};


/**
 *
 * To implement your own custom rule validation all you need to do is re-implement the function `is_valid` on `Move`
 * 
 * This function  takes the current game and the game state (which includes all the existing moves) 
 * and determines if a new candidate move is valid. Typically this will involve first matching on the move type
 * and then determining if the move is valid.
 * 
 * It function must return Ok(()) if a move is valid and Err("Some error string") for an invalid move.
 * It is useful to provide descriptive error strings as these can be visible to the end user.
 *
 */


impl Move {
    pub fn is_valid(&self, game: Game, game_state: GameState) -> Result<(), String> {
        // Check if a move is valid given the current game and its state
    }
}
