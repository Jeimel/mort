use std::{hint::black_box, time::Duration};

use criterion::{Criterion, criterion_group, criterion_main};

macro_rules! perft {
    ($name:literal, $fen:literal, $depth:literal, $criterion:expr) => {
        let pos = mort::Position::from_fen($fen).unwrap();
        $criterion.bench_function($name, |b| b.iter(|| pos.perft::<false>(black_box($depth))));
    };
}

pub fn perft(c: &mut Criterion) {
    perft!(
        "pos1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        5,
        c
    );

    perft!(
        "pos2",
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        4,
        c
    );

    perft!(
        "pos6",
        "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
        4,
        c
    );
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(100).measurement_time(Duration::new(60, 0));
    targets = perft
}
criterion_main!(benches);
