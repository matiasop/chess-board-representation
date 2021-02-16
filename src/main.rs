mod logic;
mod debugging;
mod structs;

fn main() {
    let game = logic::initialize_game();
    debugging::print_board(&game);
    debugging::print_pieces_arrays(&game);
}
