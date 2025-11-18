use crate::chess::{GenerationType, MoveList, Position};

pub fn perft<const ROOT: bool>(pos: &mut Position, depth: u16) -> usize {
    if depth == 0 {
        return 1;
    }

    let mut moves = MoveList::new();
    let mut nodes = 0;

    pos.generate::<{ GenerationType::All }>(&mut moves);

    for mov in moves.iter() {
        if !pos.legal(mov) {
            continue;
        }

        pos.make_move(mov);
        let child_nodes = perft::<false>(pos, depth - 1);
        pos.unmake_move(mov);

        nodes += child_nodes;

        if ROOT {
            println!("{}: {}", mov, child_nodes);
        }
    }

    if ROOT {
        println!("\nNodes searched: {}", nodes);
    }

    nodes
}

#[cfg(test)]
mod tests {
    use crate::{
        FEN,
        chess::{Position, perft},
    };

    const EXPECTED: [(usize, u16); 6] = [
        (119060324, 6),
        (193690690, 5),
        (178633661, 7),
        (706045033, 6),
        (89941194, 5),
        (164075551, 5),
    ];

    #[test]
    fn movegen() {
        for (fen, (nodes, depth)) in FEN.iter().zip(EXPECTED) {
            let mut pos = Position::from_fen(fen).unwrap();
            assert_eq!(nodes, perft::<false>(&mut pos, depth));
        }
    }
}
