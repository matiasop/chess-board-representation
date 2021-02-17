#[cfg(test)]
mod tests {
    use crate::debugging::*;
    use crate::logic::*;
    use crate::moves::*;
    use crate::structs::{Piece, PieceType, Pos};

    #[test]
    fn move_pawn_forward() {
        let mut game = initialize_game();

        // white pawns
        for i in 0..8 {
            let pos = Pos { x: i, y: 1 };
            let moves = game.find_possible_moves(pos, true);
            let real_moves = [make_pos(i, 2), make_pos(i, 3)];
            assert_eq!(moves, real_moves);
        }

        // black pawns
        for i in 0..8 {
            let pos = Pos { x: i, y: 6 };
            let moves = game.find_possible_moves(pos, false);
            let real_moves = [make_pos(i, 5), make_pos(i, 4)];
            assert_eq!(moves, real_moves);
        }
    }

    #[test]
    fn pawn_captures_diagonal_white() {
        let mut game = initialize_game();

        // Move black pawns in front of white pawns
        for i in 0..8 {
            game.move_piece(make_pos(i, 6), make_pos(i, 2), false);
        }

        // Check correct moves for white middle pawns
        for i in 1..7 {
            assert_eq!(
                game.find_possible_moves(make_pos(i, 1), true),
                [make_pos(i - 1, 2), make_pos(i + 1, 2)]
            );
        }

        // Check correct moves for white side pawns (x = 0 and x = 7)
        assert_eq!(
            game.find_possible_moves(make_pos(0, 1), true),
            [make_pos(1, 2)]
        );
        assert_eq!(
            game.find_possible_moves(make_pos(7, 1), true),
            [make_pos(6, 2)]
        );
    }

    #[test]
    fn pawn_captures_diagonal_black() {
        let mut game = initialize_game();

        // Move white pawns in front of black pawns
        for i in 0..8 {
            game.move_piece(make_pos(i, 1), make_pos(i, 5), true);
        }

        // Check correct moves for black middle pawns
        for i in 1..7 {
            assert_eq!(
                game.find_possible_moves(make_pos(i, 6), false),
                [make_pos(i - 1, 5), make_pos(i + 1, 5)]
            );
        }

        // Check correct moves for black side pawns (x = 0 and x = 7)
        assert_eq!(
            game.find_possible_moves(make_pos(0, 6), false),
            [make_pos(1, 5)]
        );
        assert_eq!(
            game.find_possible_moves(make_pos(7, 6), false),
            [make_pos(6, 5)]
        );
    }

    #[test]
    fn rook_moves() {
        let mut game = initialize_game();

        let from = Pos { x: 0, y: 0 };
        let to = Pos { x: 4, y: 4 };
        game.move_piece(from, to, true);

        let moves = game.find_possible_moves(to, true);
        let real_moves = [
            make_pos(5, 4),
            make_pos(6, 4),
            make_pos(7, 4),
            make_pos(3, 4),
            make_pos(2, 4),
            make_pos(1, 4),
            make_pos(0, 4),
            make_pos(4, 5),
            make_pos(4, 3),
            make_pos(4, 2),
        ];
        assert_eq!(moves, real_moves);
    }
}
