#![cfg_attr(not(any(test, feature = "serde_support")), no_std)]

pub mod string;

use string::*;

#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct GameState {}

impl GameState {}

#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum GameMessage {
    ClientConnect { id: String },
}

pub enum GameOutput {}
impl GameOutput {
    pub fn from_data(data: &[u8]) -> Option<Self> {
        todo!()
    }
}

pub enum GameInput {}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let x = String::new();
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
