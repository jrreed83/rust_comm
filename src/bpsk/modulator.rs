use std::f64::consts::{PI};
use std::cell::Cell;

trait State<T,U> {
    fn step (&mut self, x:T) -> U;
    fn reset(&mut self) -> ();
}


pub fn symbol_map(symbol:u8) -> f64 {
    let y = match symbol {
        0 => -1.0,
        1 =>  1.0,
        _ =>  0.0
    };
    y
}

pub fn map_symbols(input: &Vec<u8>) -> Vec<f64> {
    input.iter()
         .map(|&x| symbol_map(x))
         .collect()
}


pub struct Upconverter {
    state: f64,
    ratio: f64,
}

impl Upconverter {
    pub fn new(fs:f64, fc:f64) -> Upconverter {
        Upconverter {
            ratio: fc / fs,
            state: 0.0,
        }
    }

    pub fn step(&mut self,input: f64) -> f64 {
        let output = input * f64::cos(2.0*PI*self.state);
        self.state += self.ratio;
        output       
    }

    pub fn reset(&mut self) -> () {
        self.state = 0.0;
    }
}

