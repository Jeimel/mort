use std::{env, fs, path::Path};

#[allow(dead_code)]
#[path = "./src/chess/types"]
mod chess {
    mod file;
    mod rank;
    mod slider;
    mod square;
    mod squareset;

    pub use file::File;
    pub use rank::Rank;
    pub use slider::{BISHOP, ROOK, Slider, magic};
    pub use square::Square;
    pub use squareset::SquareSet;
}

use chess::{
    BISHOP, ROOK, Slider, Square, SquareSet,
    magic::{LOOKUP_TABLE_SIZE, bishop_magic_index, rook_magic_index},
};

fn write(file: &str, code: &String) {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join(file);
    fs::write(dest_path, code).unwrap();
}

fn write_table(
    table: &mut [SquareSet],
    slider: &Slider,
    index: impl Fn(Square, SquareSet) -> usize,
) {
    for sq in Square::iter() {
        let mask = slider.blockers(sq);

        for blockers in mask.iter_subset() {
            table[index(sq, blockers)] = slider.moves(sq, blockers);
        }
    }
}

fn write_slider() {
    let mut table = [SquareSet::EMPTY; LOOKUP_TABLE_SIZE];
    write_table(&mut table, &ROOK, rook_magic_index);
    write_table(&mut table, &BISHOP, bishop_magic_index);

    let code = format!(
        "const SLIDING_MOVES: &[SquareSet; crate::chess::magic::LOOKUP_TABLE_SIZE] = &{:?};",
        table
    );

    write("sliding_moves.rs", &code);
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/chess/types/file.rs");
    println!("cargo:rerun-if-changed=src/chess/types/rank.rs");
    println!("cargo:rerun-if-changed=src/chess/types/slider.rs");
    println!("cargo:rerun-if-changed=src/chess/types/slider/magic.rs");
    println!("cargo:rerun-if-changed=src/chess/types/square.rs");
    println!("cargo:rerun-if-changed=src/chess/types/squareset.rs");

    write_slider();
}
