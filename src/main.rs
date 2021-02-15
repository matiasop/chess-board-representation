mod board_repr;

fn main() {
    let board = board_repr::populate_board();

    board_repr::print_board(&board);
}
