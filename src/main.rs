mod bpsk;
mod qpsk;

extern crate num;
use num::Complex;

fn main() {
    let x: Complex<f64> = qpsk::modulator::symbol_map(0);
    println!("{0}",x);
}
