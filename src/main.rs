
extern crate rust_comm;
use rust_comm::bpsk;

fn main() {
    let v:Vec<u8> = vec![0,1,0,1,0];

    let x = bpsk::modulator::map_symbols(&v);

    for xi in &x {
        println!("{}", xi);
    }

    println!("{}", 3.14159_f32.sin());    
}
