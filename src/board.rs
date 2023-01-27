use super::{Piece, Player, Square};

pub struct Board {
    a: [Option<(Piece, Player)>; 8],
    b: [Option<(Piece, Player)>; 8],
    c: [Option<(Piece, Player)>; 8],
    d: [Option<(Piece, Player)>; 8],
    e: [Option<(Piece, Player)>; 8],
    f: [Option<(Piece, Player)>; 8],
    g: [Option<(Piece, Player)>; 8],
    h: [Option<(Piece, Player)>; 8],
}
impl Board {
    pub fn new() -> Board {
        Board {
            a: [
                Some((Piece::Rook, Player::White)),
                Some((Piece::Pawn, Player::White)),
                None,
                None,
                None,
                None,
                Some((Piece::Pawn, Player::Black)),
                Some((Piece::Rook, Player::Black)),
            ],
            b: [
                Some((Piece::Knight, Player::White)),
                Some((Piece::Pawn, Player::White)),
                None,
                None,
                None,
                None,
                Some((Piece::Pawn, Player::Black)),
                Some((Piece::Knight, Player::Black)),
            ],
            c: [
                Some((Piece::Bishop, Player::White)),
                Some((Piece::Pawn, Player::White)),
                None,
                None,
                None,
                None,
                Some((Piece::Pawn, Player::Black)),
                Some((Piece::Bishop, Player::Black)),
            ],
            d: [
                Some((Piece::Queen, Player::White)),
                Some((Piece::Pawn, Player::White)),
                None,
                None,
                None,
                None,
                Some((Piece::Pawn, Player::Black)),
                Some((Piece::Queen, Player::Black)),
            ],
            e: [
                Some((Piece::King, Player::White)),
                Some((Piece::Pawn, Player::White)),
                None,
                None,
                None,
                None,
                Some((Piece::Pawn, Player::Black)),
                Some((Piece::King, Player::Black)),
            ],
            f: [
                Some((Piece::Bishop, Player::White)),
                Some((Piece::Pawn, Player::White)),
                None,
                None,
                None,
                None,
                Some((Piece::Pawn, Player::Black)),
                Some((Piece::Bishop, Player::Black)),
            ],
            g: [
                Some((Piece::Knight, Player::White)),
                Some((Piece::Pawn, Player::White)),
                None,
                None,
                None,
                None,
                Some((Piece::Pawn, Player::Black)),
                Some((Piece::Knight, Player::Black)),
            ],
            h: [
                Some((Piece::Rook, Player::White)),
                Some((Piece::Pawn, Player::White)),
                None,
                None,
                None,
                None,
                Some((Piece::Pawn, Player::Black)),
                Some((Piece::Rook, Player::Black)),
            ],
        }
    }
    pub fn move_piece(&mut self, origin: Square, destination: Square) {
        todo!()
    }
}
