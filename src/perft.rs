use crate::Position;

pub fn perft<const ROOT: bool>(pos: &mut Position, depth: usize) -> usize {
    if depth == 0 {
        return 1;
    }

    let mut nodes = 0;

    for mov in pos.gen_moves() {
        if !pos.make_move(mov) {
            continue;
        }

        let child_nodes = perft::<false>(pos, depth - 1);
        nodes += child_nodes;

        if ROOT {
            println!("{}: {}", mov, child_nodes);
        }

        pos.unmake_move();
    }

    if ROOT {
        println!("\nNodes searched: {}", nodes);
    }

    nodes
}

#[cfg(test)]
mod tests {
    macro_rules! perft {
        ($name:ident, $fen:literal, $result:literal, $depth:literal) => {
            #[test]
            fn $name() {
                let mut pos = crate::Position::from_fen($fen).unwrap();
                assert_eq!($result, crate::perft::<false>(&mut pos, $depth));
            }
        };
    }

    perft!(
        pos1,
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        119060324,
        6
    );

    perft!(
        pos2,
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        193690690,
        5
    );

    perft!(
        pos3,
        "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1",
        178633661,
        7
    );

    perft!(
        pos4,
        "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
        706045033,
        6
    );

    perft!(
        pos5,
        "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
        89941194,
        5
    );

    perft!(
        pos6,
        "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
        164075551,
        5
    );
}
