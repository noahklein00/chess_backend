const FILE_A: u64 = 0x0101010101010101;
const FILE_H: u64 = 0x8080808080808080;
const RANK_1: u64 = 0x00000000000000FF;
const RANK_8: u64 = 0xFF00000000000000;
const DIAGONAL: u64 = 0x8040201008040201;
const ANTI_DIAGONAL: u64 = 0x0102040810204080;
const WHITE_SQUARES: u64 = 0x55AA55AA55AA55AA;
const BLACK_SQUARES: u64 = 0xAA55AA55AA55AA55;

pub fn north_one(pieces: u64) -> u64 { pieces << 8 }
pub fn north_east_one(pieces: u64) -> u64 { pieces << 9 & !FILE_A }
pub fn east_one(pieces: u64) -> u64 { pieces << 1 & !FILE_A }
pub fn south_east_one(pieces: u64) -> u64 { pieces >> 7 & !FILE_A }
pub fn south_one(pieces: u64) -> u64 { pieces >> 8 }
pub fn south_west_one(pieces: u64) -> u64 { pieces >> 9 & !FILE_H }
pub fn west_one(pieces: u64) -> u64 { pieces >> 1 & !FILE_H }
pub fn north_west_one(pieces: u64) -> u64 { pieces << 7 & !FILE_H }

#[test]
fn check_move_north() {
    let state: u64 = 0x00000000FF000000;
    let result: u64 = north_one(state);
    assert_eq!(result, 0x000000FF00000000);

    let state: u64 = 0xFF00000000000000;
    let result: u64 = north_one(state);
    assert_eq!(result, 0x0000000000000000);
}

#[test]
fn check_move_north_east() {
    let state: u64 = 0x000000FF00000000;
    let result: u64 = north_east_one(state);
    assert_eq!(result, 0x0000FE0000000000);
}

#[test]
fn check_move_east() {
    let state: u64 = 0x000000FF00000000;
    let result: u64 = east_one(state);
    assert_eq!(result, 0x000000FE00000000);
}

#[test]
fn check_move_south_east() {
    let state: u64 = 0x000000FF00000000;
    let result: u64 = south_east_one(state);
    assert_eq!(result, 0x00000000FE000000);
}

#[test]
fn check_move_south() {
    let state: u64 = 0x000000FF00000000;
    let result: u64 = south_one(state);
    assert_eq!(result, 0x00000000FF000000);

    let state: u64 = 0x00000000000000FF;
    let result: u64 = south_one(state);
    assert_eq!(result, 0x0000000000000000);
}

#[test]
fn check_move_south_west() {
    let state: u64 = 0x000000FF00000000;
    let result: u64 = south_west_one(state);
    assert_eq!(result, 0x000000007F000000);
}

#[test]
fn check_move_west() {
    let state: u64 = 0x000000FF00000000;
    let result: u64 = west_one(state);
    assert_eq!(result, 0x0000007F00000000);
}

#[test]
fn check_move_north_west() {
    let state: u64 = 0x000000FF00000000;
    let result: u64 = north_west_one(state);
    assert_eq!(result, 0x00007F0000000000);
}