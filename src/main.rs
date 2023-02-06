mod moves;
mod utility;
mod pieces;

use crate::pieces::pawn;

fn main() {
    let w_pawns: u64 = 0x000000000102FC00;
    let empty: u64 = 0xFFFFFFFFFFFFFFFF;
    let moveset: u64 = pawn::w_single_push_squares(w_pawns, empty);
    utility::print_as_board(w_pawns);
    utility::print_as_board(moveset);
    let moveset: u64 = pawn::w_double_push_squares(w_pawns, empty);
    utility::print_as_board(moveset);
    for i in 0..64 {
        utility::print_as_board(pieces::knight::KNIGHT_MOVES[i]);
    }
}
