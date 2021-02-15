use crate::logic::*;

pub fn print_board(board: &[[Square; 8]; 8]) {
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

pub fn print_piece_color(square: &Square) {
    match square.piece {
        Some(p) => println!("{}", p.color),
        None => panic!("Error: could not print piece color"),
    }
}