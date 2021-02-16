#[cfg(test)]
mod tests {
    use crate::structs::{Piece, PieceType, Pos};
    use crate::debugging::*;
    use crate::moves::*;
    use crate::logic::*;

    #[test]
    fn it_works() {
        // assert_eq!(2 + 2, 4);
        // let mut game = initialize_game();
        assert_eq!(Pos { x: 1, y: 1}, Pos {x: 1, y: 1});
    }
}
