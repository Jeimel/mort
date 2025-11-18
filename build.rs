use std::{env, fs, path::Path};

use types::{
    File, Rank, Square, SquareSet, const_for,
    slider::{
        BISHOP, ROOK, Slider,
        magic::{LOOKUP_TABLE_SIZE, bishop_magic_index, rook_magic_index},
    },
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
        "const SLIDING_MOVES: &[types::SquareSet; {LOOKUP_TABLE_SIZE}] = &{:?};",
        table
    );

    write("sliding_moves.rs", &code);
}

fn line() -> [[SquareSet; 64]; 64] {
    const fn line(i: usize, j: usize) -> u64 {
        const DIAGONALS: [u64; 15] = [
            0x0100_0000_0000_0000,
            0x0201_0000_0000_0000,
            0x0402_0100_0000_0000,
            0x0804_0201_0000_0000,
            0x1008_0402_0100_0000,
            0x2010_0804_0201_0000,
            0x4020_1008_0402_0100,
            0x8040_2010_0804_0201,
            0x0080_4020_1008_0402,
            0x0000_8040_2010_0804,
            0x0000_0080_4020_1008,
            0x0000_0000_8040_2010,
            0x0000_0000_0080_4020,
            0x0000_0000_0000_8040,
            0x0000_0000_0000_0080,
        ];

        let (sq, file, rank) = (1u64 << j, i % 8, i / 8);

        let diagonal = DIAGONALS[7 + file - rank];
        if (diagonal & sq) != 0 {
            return diagonal;
        }

        let anti_diagonal = DIAGONALS[file + rank].swap_bytes();
        if (anti_diagonal & sq) != 0 {
            return anti_diagonal;
        }

        let file = File::new(file as u8).unwrap().set().0;
        if (file & sq) != 0 {
            return file;
        }

        let rank = Rank::new(rank as u8).unwrap().set().0;
        if (rank & sq) != 0 {
            return rank;
        }

        0
    }

    let mut result = [[SquareSet::EMPTY; 64]; 64];

    const_for!(let mut i = 0; i < 64; i += 1; {
        const_for!(let mut j = 0; j < 64; j += 1; {
            result[i][j] = SquareSet(line(i, j));
        });
    });

    result
}

fn between(line: &[[SquareSet; 64]; 64]) -> [[SquareSet; 64]; 64] {
    const fn between(line: &[[SquareSet; 64]; 64], i: usize, j: usize) -> u64 {
        let (a, b) = (Square::new(i as u8).unwrap(), Square::new(j as u8).unwrap());

        let rook = ROOK.moves(a, b.set()).0 & ROOK.moves(b, a.set()).0;
        let bishop = BISHOP.moves(a, b.set()).0 & BISHOP.moves(b, a.set()).0;

        ((rook | bishop) & line[i][j].0) | b.set().0
    }

    let mut result = [[SquareSet::EMPTY; 64]; 64];

    const_for!(let mut i = 0; i < 64; i += 1; {
        const_for!(let mut j = 0; j < 64; j += 1; {
            result[i][j] = SquareSet(between(line, i, j));
        });
    });

    result
}

fn write_squareset() {
    let line = line();
    let between = between(&line);

    let mut code = String::new();

    code.push_str(&format!(
        "pub const LINE: &[[SquareSet; 64]; 64] = &{:?};",
        line
    ));

    code.push_str(&format!(
        "pub const BETWEEN: &[[SquareSet; 64]; 64] = &{:?};",
        between
    ));

    write("squareset_tables.rs", &code);
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=types/src/slider/magic.rs");
    println!("cargo:rerun-if-changed=types/src/slider.rs");

    write_slider();
    write_squareset();
}
