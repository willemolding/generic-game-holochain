use hdk::holochain_core_types::{
    error::HolochainError,
    json::JsonString,
};

/**
 *
 * The MoveType enum defines all the types of moves that are valid in your game and the 
 * data they carry. In Checkers you can move a piece (MovePiece) from a location to another location.
 *
 */

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson, PartialEq)]
pub enum MoveType {
    Place {x: usize, y: usize},
}

impl MoveType {
	pub fn describe() -> Vec<MoveType> {
		vec![
			MoveType::Place{x: 0, y: 0}
			// add the other variants here to add descriptors
		]
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson, PartialEq)]
pub struct Piece {
    pub x: usize,
    pub y: usize,
}
