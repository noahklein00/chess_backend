mod moves;
mod utility;
mod pieces;

fn main() {
    // let game = Board::new();
    // game.print();

    let w_pawns: u64 = 0x000000000102FC00;
    let empty: u64 = 0xFFFFFFFFFFFFFFFF;
    let moveset: u64 = pieces::w_single_push_pawns(w_pawns, empty);
    utility::print_as_board(w_pawns);
    utility::print_as_board(moveset);
    let moveset: u64 = pieces::w_double_push_pawns(w_pawns, empty);
    utility::print_as_board(moveset);
}
