pub fn print_as_board(pieces: u64) {
    println!("   a b c d e f g h ");
    println!("  +-+-+-+-+-+-+-+-+");
        for rank in (0..8).rev() {
            print!("{} ", rank + 1);
            for file in 0..8 {
                let idx = rank * 8 + file;
                let bit = 1 << idx;
                let piece = 
                    if bit & pieces != 0 {
                        "x"
                    } else {
                        " "
                    };
                print!("|{}", piece);
            }
            println!("| {}", rank + 1);
        }
    println!("  +-+-+-+-+-+-+-+-+");
    println!("   a b c d e f g h ");
}