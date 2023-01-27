mod board;
mod parser;

pub use board::Board;

pub struct Square(char, u8);

#[derive(Debug)]
pub enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}
#[derive(Debug)]
pub enum Player {
    White,
    Black,
}
