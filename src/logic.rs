use crate::structs::{PieceType, Piece, Pos};

#[derive(Copy, Clone)]
pub struct Square {
    pub piece: Option<Piece>,
}

impl Square {
    fn get_piece(&self) -> Piece {
        match self.piece {
            Some(p) => p,
            None => panic!("Error: could not get piece"),
        }
    }
    
    fn remove_piece(&mut self) {
        self.piece = None;
    }
}

pub struct Game {
    pub board: [[Square; 8]; 8],
    pub white_pieces: [Piece; 16],
    pub black_pieces: [Piece; 16],
}

pub fn initialize_game() -> Game {
    // Initialize empty board
    let mut board: [[Square; 8]; 8] = [[Square { piece: None }; 8]; 8];

    // Initialize empy arrays of pieces
    let mut white_pieces: [Piece; 16] = [Piece {piece_type: PieceType::Pawn, color: true, alive: false, position: Pos { x: 100, y: 100}}; 16];
    let mut black_pieces: [Piece; 16] = [Piece {piece_type: PieceType::Pawn, color: true, alive: false, position: Pos { x: 100, y: 100}}; 16];

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
                    alive: true,
                    position: Pos {x: i, y: j},
                }),
                // Rooks
                (0, 0) | (7, 0) | (0, 7) | (7, 7) => Some(Piece {
                    piece_type: PieceType::Rook,
                    color: color,
                    alive: true,
                    position: Pos {x: i, y: j},
                }),
                // Bishops
                (1, 0) | (6, 0) | (1, 7) | (6, 7) => Some(Piece {
                    piece_type: PieceType::Bishop,
                    color: color,
                    alive: true,
                    position: Pos {x: i, y: j},
                }),
                // Kinghts
                (2, 0) | (5, 0) | (2, 7) | (5, 7) => Some(Piece {
                    piece_type: PieceType::Knight,
                    color: color,
                    alive: true,
                    position: Pos {x: i, y: j},
                }),
                // Queens
                (3, 0) | (3, 7) => Some(Piece {
                    piece_type: PieceType::Queen,
                    color: color,
                    alive: true,
                    position: Pos {x: i, y: j},
                }),
                // King
                (4, 0) | (4, 7) => Some(Piece {
                    piece_type: PieceType::King,
                    color: color,
                    alive: true,
                    position: Pos {x: i, y: j},
                }),
                _ => None,
            };

            // Fill Board
            board[i][j] = Square { piece: piece };
            
            // Fill array of pieces
            match piece {
                Some(_p) => {
                    if j <= 2 {
                        white_pieces[i + 8*j] = piece.unwrap();
                    } else if j >= 6 {
                        black_pieces[i + 8*(7 - j)] = piece.unwrap();
                    }
                },
                None => (),
            }
        }
    }
    return Game {board, white_pieces, black_pieces}
}

pub fn move_piece(board: &mut [[Square; 8]; 8], from: Pos, to: Pos) {
    let mut initial_square = board[from.x][from.y];
    let mut final_square = board[to.x][to.y];

    // Get piece
    let piece = initial_square.get_piece();

    // Move piece to final position
    final_square.piece = Some(piece);

    // Remove piece original position
    initial_square.remove_piece();

    // Make changes in the board
    board[from.x][from.y] = initial_square;
    board[to.x][to.y] = final_square;
}
