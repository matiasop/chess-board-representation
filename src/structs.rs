use std::cmp::PartialEq;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Copy, Clone)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: bool,
    pub alive: bool,
    pub position: Pos,
}

#[derive(Copy, Clone, Debug)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

