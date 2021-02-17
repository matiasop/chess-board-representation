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

pub fn make_pos(x: usize, y: usize) -> Pos {
    Pos { x, y }
}

/// Returns a vector of positions with all the valid pawn moves
pub fn check_pawn_moves(game: &Game, piece: &Piece) -> Vec<Pos> {
    let mut moves: Vec<Pos> = Vec::new();

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
    let (x_diag_left, y_diag_left) = (x_pos - 1, y_pos + increment);
    let (x_diag_right, y_diag_right) = (x_pos + 1, y_pos + increment);
    // Left diagonal
    if valid_numbers(x_diag_left, y_diag_left) {
        let left_diag_pos = make_pos(
            x_diag_left.try_into().unwrap(),
            y_diag_left.try_into().unwrap(),
        );
        if piece_in_pos(&game, left_diag_pos) {
            moves.push(left_diag_pos);
        }
    }
    // Right diagonal
    if valid_numbers(x_diag_right, y_diag_right) {
        let right_diag_pos = make_pos(
            x_diag_right.try_into().unwrap(),
            y_diag_right.try_into().unwrap(),
        );
        if piece_in_pos(&game, right_diag_pos) {
            moves.push(right_diag_pos);
        }
    }

    // Check en passant
    println!("{:?}", moves);
    moves
}

pub fn check_rook_moves(game: &Game, piece: &Piece) -> Vec<Pos> {
    let mut moves: Vec<Pos> = Vec::new();

    let x_pos: isize = piece.position.x.try_into().unwrap();
    let y_pos: isize = piece.position.y.try_into().unwrap();

    // Check in all four directions if there is a piece
    for (inc_x, inc_y) in [(1, 0), (-1, 0), (0, 1), (0, -1)].iter() {
        for i in 1..8 {
            let (x_new_pos, y_new_pos) = (x_pos + inc_x * i, y_pos + inc_y * i);
            if valid_numbers(x_new_pos, y_new_pos) {
                let new_pos =
                    make_pos(x_new_pos.try_into().unwrap(), y_new_pos.try_into().unwrap());
                if !piece_in_pos(&game, new_pos) {
                    moves.push(new_pos);
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }
    moves
}
