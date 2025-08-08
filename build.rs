use std::{env, fs, path::Path};

use types::{
    Square, SquareSet,
    slider::{
        BISHOP, ROOK, Slider,
        magic::{LOOKUP_TABLE_SIZE, bishop_magic_index, rook_magic_index},
    },
};

fn write_table(
    table: &mut [SquareSet],
    slider: Slider,
    index: impl Fn(Square, SquareSet) -> usize,
) {
    for sq in Square::iter() {
        let mask = slider.blockers(sq);

        for blockers in mask.iter_subset() {
            table[index(sq, blockers)] = slider.moves(sq, blockers);
        }
    }
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=types/src/slider/magic.rs");

    let mut table = [SquareSet::EMPTY; LOOKUP_TABLE_SIZE];
    write_table(&mut table, ROOK, rook_magic_index);
    write_table(&mut table, BISHOP, bishop_magic_index);

    let code = format!(
        "const SLIDING_MOVES: &[SquareSet; {}] = &{:?};",
        LOOKUP_TABLE_SIZE, table
    );

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("sliding_moves.rs");
    fs::write(dest_path, code).unwrap()
}
