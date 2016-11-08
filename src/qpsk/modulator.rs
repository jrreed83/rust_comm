use num::Complex;

pub fn symbol_map(symbol:u8) -> Complex<f64> {
    let y = match symbol {
        0 => Complex::new(-1.0,-1.0),
        1 => Complex::new(-1.0, 1.0),
        2 => Complex::new( 1.0,-1.0),
        3 => Complex::new( 1.0, 1.0),
        _ => Complex::new( 0.0, 0.0)
    };
    y
}