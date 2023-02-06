pub mod pawn {
    use crate::{moves, utility};

    // ---------- Can move to ----------
    // _squares functions give the set of spaces the colored pawns can move to
    pub fn w_single_push_squares(w_pawns: u64, empty: u64) -> u64 {
        moves::north_one(w_pawns) & empty
    }

    pub fn w_double_push_squares(w_pawns: u64, empty: u64) -> u64 {
        let rank4: u64 = 0x00000000FF000000;
        let single_pushes: u64 = w_single_push_squares(w_pawns, empty);
        moves::north_one(single_pushes) & empty & rank4
    }

    pub fn b_single_push_squares(b_pawns: u64, empty: u64) -> u64 {
        moves::south_one(b_pawns) & empty
    }

    pub fn b_double_push_squares(b_pawns: u64, empty: u64) -> u64 {
        let rank5: u64 = 0x000000FF00000000;
        let single_pushes: u64 = b_single_push_squares(b_pawns, empty);
        moves::south_one(single_pushes) & empty & rank5
    }

    // ---------- Can move from ----------
    // _push functions give the set of spaces of colored pawns that can push
    pub fn w_pawns_can_push(w_pawns: u64, empty: u64) -> u64 {
        moves::south_one(empty) & w_pawns
    }

    pub fn w_pawns_can_double_push(w_pawns: u64, empty: u64) -> u64 {
        let rank4: u64 = 0x00000000FF000000;
        let empty_rank3: u64 = moves::south_one(empty & rank4) & empty;
        w_pawns_can_push(w_pawns, empty_rank3) 
    }

    pub fn b_pawns_can_push(b_pawns: u64, empty: u64) -> u64 {
        moves::north_one(empty) & b_pawns
    }

    pub fn b_pawns_can_double_push(b_pawns: u64, empty: u64) -> u64 {
        let rank5: u64 = 0x000000FF00000000;
        let empty_rank6: u64 = moves::north_one(empty & rank5) & empty;
        b_pawns_can_push(b_pawns, empty_rank6) 
    }

    // ---------- Rammed pawns ----------
    // _ram functions give the set of colored pawns that are ramming other
    // colored pawns
    pub fn w_ram(w_pawns: u64, b_pawns: u64) -> u64 {
        moves::south_one(b_pawns) & w_pawns
    }

    pub fn b_ram(b_pawns: u64, w_pawns: u64) -> u64 {
        moves::north_one(w_pawns) & b_pawns
    }

    // ---------- TEST ----------
    #[test]
    fn check_single_push_squares() {
        let w_pawns: u64 = 0x000000000201FC00;
        let b_pawns: u64 = 0x00FC010200000000;
        let w_pawn_pushes: u64 = w_single_push_squares(w_pawns, !b_pawns);
        let b_pawn_pushes: u64 = b_single_push_squares(b_pawns, !w_pawns);
        assert_eq!(w_pawn_pushes, 0x0000000001FC0000);
        assert_eq!(b_pawn_pushes, 0x0000FC0100000000);
    }

    #[test]
    fn check_double_push_squares() {
        let w_pawns: u64 = 0x000020100201CC00;
        let b_pawns: u64 = 0x00F0010204080000;
        let w_pawn_pushes: u64 = w_double_push_squares(w_pawns, !b_pawns);
        let b_pawn_pushes: u64 = b_double_push_squares(b_pawns, !w_pawns);
        assert_eq!(w_pawn_pushes, 0x00000000C0000000);
        assert_eq!(b_pawn_pushes, 0x000000C000000000);
    }

    #[test]
    fn check_pawns_can_single_push() {
        let w_pawns: u64 = 0x000000010200FC00;
        let b_pawns: u64 = 0x00FC010200000000;
        let w_pawn_can_move: u64 = w_pawns_can_push(w_pawns, !b_pawns);
        let b_pawn_can_move: u64 = b_pawns_can_push(b_pawns, !w_pawns);
        assert_eq!(w_pawn_can_move, 0x000000000000FC00);
        assert_eq!(b_pawn_can_move, 0x00FC000000000000);
    }

    #[test]
    fn check_pawns_can_double_push() {
        let w_pawns: u64 = 0x000000010600F800;
        let b_pawns: u64 = 0x00F8060200000000;
        let w_pawn_can_move: u64 = w_pawns_can_double_push(w_pawns, !b_pawns);
        let b_pawn_can_move: u64 = b_pawns_can_double_push(b_pawns, !w_pawns);
        assert_eq!(w_pawn_can_move, 0x000000000000F800);
        assert_eq!(b_pawn_can_move, 0x00F8000000000000);
    }
}

pub mod knight {
    use crate::{moves, utility};

    const fn knight_uur(n: u64) -> u64 { (n << 17) & !moves::FILE_A }
    const fn knight_urr(n: u64) -> u64 { (n << 10) & !(moves::FILE_A | moves::FILE_B) }
    const fn knight_drr(n: u64) -> u64 { (n >>  6) & !(moves::FILE_A | moves::FILE_B) }
    const fn knight_ddr(n: u64) -> u64 { (n >> 15) & !moves::FILE_A }
    const fn knight_ddl(n: u64) -> u64 { (n << 15) & !moves::FILE_H }
    const fn knight_dll(n: u64) -> u64 { (n <<  6) & !(moves::FILE_G | moves::FILE_H) }
    const fn knight_ull(n: u64) -> u64 { (n >> 10) & !(moves::FILE_G | moves::FILE_H) }
    const fn knight_uul(n: u64) -> u64 { (n >> 17) & !moves::FILE_H }

    const fn generate_knight_moves() -> [u64; 64] {
        let mut moves = [0; 64];
        let mut idx = 0;
        while idx < 64 {
            let loc: u64 = 1 << idx;
            moves[idx] |= knight_uur(loc);
            moves[idx] |= knight_urr(loc);
            moves[idx] |= knight_drr(loc);
            moves[idx] |= knight_ddr(loc);
            moves[idx] |= knight_ddl(loc);
            moves[idx] |= knight_dll(loc);
            moves[idx] |= knight_ull(loc);
            moves[idx] |= knight_uul(loc);
            idx += 1;
        }
        moves
    }

    pub const KNIGHT_MOVES: [u64; 64] = generate_knight_moves();
}