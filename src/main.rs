fn main() {
    println!("1:{}; 2:{}; 50:{}; 300:{};", fibbonaci(1), fibbonaci(2), fibbonaci(50),fibbonaci(300))
}

const GOLDEN_RATIO:f64 = 1.618033988749894848204586834365638117720309179805762862135;
const NEGATIV_GOLDEN_RATIO:f64 = -0.61803398874989484820458683436563811772030917980576286213;
const FIVE:f64 = 5.0_f64;

fn fibbonaci(n:i32) -> f64 {
    let result = (GOLDEN_RATIO.powi(n) - NEGATIV_GOLDEN_RATIO.powi(n)) / (FIVE.sqrt());
    result.round()
}