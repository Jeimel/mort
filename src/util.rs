mod arrayvec;
mod bench;
mod perft;
mod rng;

pub use arrayvec::ArrayVec;
pub use bench::bench;
pub use perft::perft;
pub use rng::XorShiftState;

#[macro_export]
macro_rules! const_for {
    ($init:stmt; $condition:expr; $next: expr; $body:block) => {
        $init
        while $condition {
            $body;
            $next;
        }
    };
}
