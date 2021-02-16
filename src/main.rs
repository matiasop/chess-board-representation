mod debugging;
mod logic;
mod structs;
mod tests;

fn main() {
    let mut game = logic::initialize_game();
    // debugging::print_board(&game);
    // debugging::print_pieces_arrays(&game);

    // Move piece
    let from = structs::Pos { x: 0, y: 1 };
    let to = structs::Pos { x: 0, y: 2 };
    game.move_piece(from, to, true);
    debugging::print_board(&game);
    debugging::print_pieces_arrays(&game);
}
