use chess_rs::{Board, Square};

fn main() {
    let mut board = Board::new();

    board.move_piece(
        Square::try_from(('g', 1)).unwrap(),
        Square::try_from(('f', 3)).unwrap(),
    );
    board.move_piece("g1".parse().unwrap(), "f3".parse().unwrap());
}
