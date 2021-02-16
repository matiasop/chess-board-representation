use std::convert::TryInto;

use crate::structs::{Piece, PieceType, Pos};
// use crate::debugging::*;
use crate::logic::*;

/// Returns true if there is a piece in position. False otherwise
fn piece_in_pos(game: &Game, position: Pos) -> bool {
    let square = game.board[position.x][position.y];
    match square.piece {
        Some(_) => true,
        None => false,
    }
}

/// Returns true if position is a valid chess position
fn valid_pos(position: Pos) -> bool {
    position.x < 8 && position.y < 8
}

/// Returns true if number is between 0 and 8
fn valid_numbers(x: isize, y: isize) -> bool {
    0 <= x && x < 8 && 0 <= y && y < 8
}

fn make_pos(x: usize, y: usize) -> Pos {
    Pos { x, y }
}

/// Returns a vector of positions with all the valid pawn moves
pub fn check_pawn_moves(game: &Game, piece: &Piece) -> Vec<Pos> {
    let mut moves: Vec<Pos> = Vec::new();
    // let check_if_no_piece: Vec<Pos> = Vec::new();
    // let check_if_piece: Vec<Pos> = Vec::new();

    // White pawns advance in the positive 'j' direction
    // Black pawns advance in the negative 'j' direction
    let increment: isize = match piece.color {
        true => 1,
        false => -1,
    };

    let x_pos: isize = piece.position.x.try_into().unwrap();
    let y_pos: isize = piece.position.y.try_into().unwrap();

    // Check piece in front
    let (x_front, y_front) = (x_pos, y_pos + increment);
    if valid_numbers(x_front, y_front) {
        let front_pos = make_pos(x_front.try_into().unwrap(), y_front.try_into().unwrap());
        if !piece_in_pos(&game, front_pos) {
            moves.push(front_pos);
        }
    }

    // Check if pawn is in its initial position
    if piece.color && y_pos == 1 || !piece.color && y_pos == 6 {
        let (x_two_front, y_two_front) = (x_pos, y_pos + 2 * increment);
        if valid_numbers(x_two_front, y_two_front) && valid_numbers(x_front, y_front) {
            let two_front_pos = make_pos(
                x_two_front.try_into().unwrap(),
                y_two_front.try_into().unwrap(),
            );
            let front_pos = make_pos(x_front.try_into().unwrap(), y_front.try_into().unwrap());
            if !piece_in_pos(&game, front_pos) && !piece_in_pos(&game, two_front_pos) {
                moves.push(two_front_pos);
            }
        }
    }
    // Check if there is an adversary piece in the pawn's forward diagonals

    // Check en passant
    println!("{:?}", moves);
    moves
}
