
pub fn symbol_map(symbol:u8) -> f64 {
    let y = match symbol {
        0 => -1.0_f64,
        1 =>  1.0_f64,
        _ =>  0.0_f64
    };
    y
}

pub fn map_symbols(input: &Vec<u8>) -> Vec<f64> {
    input.iter()
         .map(|&x| symbol_map(x))
         .collect()
}