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
}

#[derive(Copy, Clone)]
pub struct Square {
    pub piece: Option<Piece>,
}

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

pub fn populate_board() -> [[Square; 8]; 8] {
    // Initialize empty board
    let mut board: [[Square; 8]; 8] = [[Square { piece: None }; 8]; 8];

    // Populate board
    for i in 0..8 {
        for j in 0..8 {
            // Piece Color
            let color = j < 4;
            // Piece Type
            let piece: Option<Piece> = match (i, j) {
                // Pawns
                (_, 1) | (_, 6) => Some(Piece {
                    piece_type: PieceType::Pawn,
                    color: color,
                }),
                // Rooks
                (0, 0) | (7, 0) | (0, 7) | (7, 7) => Some(Piece {
                    piece_type: PieceType::Rook,
                    color: color,
                }),
                // Bishops
                (1, 0) | (6, 0) | (1, 7) | (6, 7) => Some(Piece {
                    piece_type: PieceType::Bishop,
                    color: color,
                }),
                // Kinghts
                (2, 0) | (5, 0) | (2, 7) | (5, 7) => Some(Piece {
                    piece_type: PieceType::Knight,
                    color: color,
                }),
                // Queens
                (3, 0) | (3, 7) => Some(Piece {
                    piece_type: PieceType::Queen,
                    color: color,
                }),
                // King
                (4, 0) | (4, 7) => Some(Piece {
                    piece_type: PieceType::King,
                    color: color,
                }),
                _ => None,
            };

            // Fill Board
            board[i][j] = Square { piece: piece };
        }
    }
    board
}
