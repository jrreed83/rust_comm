
extern crate rust_comm;
use rust_comm::bpsk;


fn main() {
    let v:Vec<u8> = vec![0,1,0,1,0];

    let x = bpsk::modulator::map_symbols(&v);

    let mut upconverter = bpsk::modulator::Upconverter::new(1000.0,10.0);


    for i in 1..50 {
        println!("{}",upconverter.step());
    }
  
}
