
pub fn symbol_map(symbol:u8) -> f64 {
    let y = match symbol {
        0 => -1.0,
        1 =>  1.0,
        _ =>  1.0
    };
    y
}