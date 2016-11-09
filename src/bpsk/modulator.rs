
use std::f64::consts::{PI};
use std::cell::Cell;

pub fn symbol_map(symbol:u8) -> f64 {
    let y = match symbol {
        0 => -1.0_f64,
        1 =>  1.0_f64,
        _ =>  0.0_f64
    };
    y
}

pub fn map_symbols(input: &Vec<u8>) -> Vec<f64> {
    input.into_iter()
         .map(|&x| symbol_map(x))
         .collect()
}

pub struct Upconverter {
    state: Cell<f64>,
    ratio: f64,
}

impl Upconverter {
    pub fn new(fs:f64, fc:f64) -> Upconverter {
        Upconverter {
            ratio: fc / fs,
            state: Cell::new(0.0),
        }
    }

    pub fn step(&self) -> f64 {
        let phase = self.state.get();
        let output = f64::cos(2.0*PI*phase);
        self.state.set(phase + self.ratio);
        output       
    }

    pub fn reset(&self) -> () {
        self.state.set(0.0);
    }
}

pub fn upconvert(ratio: f64) -> Box<FnMut() -> f64> {
    let mut state: f64 = 0.0;
    Box :: new (move || {
        let output = f64::cos(2.0*PI*state);
        state += ratio;
        output
    })
}
