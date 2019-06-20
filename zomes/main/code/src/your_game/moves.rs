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
    // <<DEVCAMP-TODO>> YOUR MOVE ENUM VARIENTS HERE
}

impl MoveType {
	pub fn describe() -> Vec<MoveType> {
		// <<DEVCAMP-TODO>> SHOULD RETURN AN EXAMPLE OF EACH VARIENT
		Vec::new()
	}
}
