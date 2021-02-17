mod debugging;
mod logic;
mod moves;
mod structs;
mod tests;

fn main() {
    let mut game = logic::initialize_game();
    // debugging::print_board(&game);
    // debugging::print_pieces_arrays(&game);

    // Move piece
    let from = structs::Pos { x: 0, y: 0 };
    let to = structs::Pos { x: 4, y: 4 };
    game.move_piece(from, to, true);
    debugging::print_board(&game);
    // debugging::print_pieces_arrays(&game);

    // Find possible moves
    let moves = game.find_possible_moves(to, true);
    for m in moves.iter() {
        println!("{:?}", m);
    }
}
