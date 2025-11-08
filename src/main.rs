#![feature(adt_const_params)]

mod chess;
mod error;
mod evaluation;
mod perft;
mod rng;
mod search;
mod uci;

fn main() {
    uci::run();
}
