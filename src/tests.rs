#[cfg(test)]
mod tests {
    use crate::debugging::*;
    use crate::logic::*;
    use crate::moves::*;
    use crate::structs::{Piece, PieceType, Pos};

    #[test]
    fn it_works() {
        assert_eq!(Pos { x: 1, y: 1 }, Pos { x: 1, y: 1 });
    }

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
}
