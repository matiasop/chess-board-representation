use crate::logic::*;
use crate::structs::*;

pub fn print_board(game: &Game) {
    let board = game.board;

    for col in 0..8 {
        for row in 0..8 {
            match board[col][row].piece {
                Some(p) => match p.color {
                    true => match p.piece_type {
                        PieceType::Pawn => print!("P"),
                        PieceType::Rook => print!("R"),
                        PieceType::Bishop => print!("B"),
                        PieceType::Knight => print!("N"),
                        PieceType::Queen => print!("Q"),
                        PieceType::King => print!("K"),
                    },
                    false => match p.piece_type {
                        PieceType::Pawn => print!("p"),
                        PieceType::Rook => print!("r"),
                        PieceType::Bishop => print!("b"),
                        PieceType::Knight => print!("n"),
                        PieceType::Queen => print!("q"),
                        PieceType::King => print!("k"),
                    },
                },
                None => print!("-"),
            }
        }
        println!();
    }
}

pub fn print_pieces_arrays(game: &Game) {
    println!("White Pieces");
    print_pieces_array(&game.white_pieces);
    println!();
    
    println!("Black Pieces");
    print_pieces_array(&game.black_pieces);
    println!();
}

fn print_pieces_array(array: &[Piece; 16]) {
    for i in 0..16 {
        let piece = array[i];
        println!("Type: {:?}, White?: {}, Alive?: {}, X: {}, Y: {}", piece.piece_type, piece.color, piece.alive, piece.position.x, piece.position.y);
    }
}

pub fn print_piece_color(square: &Square) {
    match square.piece {
        Some(p) => println!("{}", p.color),
        None => panic!("Error: could not print piece color"),
    }
}