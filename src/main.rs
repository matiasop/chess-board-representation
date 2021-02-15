mod logic;
mod debugging;

fn main() {
    let mut board = logic::populate_board();

    debugging::print_board(&board);
    println!("");

    logic::move_piece(&mut board, logic::Pos { x: 0, y: 1 }, logic::Pos { x: 0, y: 2});

    debugging::print_board(&board);
}
