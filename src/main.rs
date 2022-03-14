const PRECISION: u32 = 200;
use rug::{
    float::{Round, self},
    Float,
    ops::{AddAssignRound}
};

fn main() {
    leibniz();
}

fn ramanujan() {
    //let mut one_over_pi = Float::new();

    for k in 0..5u64 {
        //one_over_pi.add_assign_round(Round::Up);
        //println!("i: {}, pi: {}", i, pi);
        continue;
    }
}

fn leibniz() {
    let mut pi_over_4 = Float::with_val(PRECISION, 1.0);

    for i in 0..10000000000u64 {

        let mut denominator = ((i << 1) + 3) as f64;
        if i % 2 == 0 { denominator *= -1.0; }

        let fraction = Float::with_val(PRECISION, 1.0 / denominator);
        //println!("frac:{}", fraction);
        pi_over_4.add_assign_round(fraction, Round::Up);

        let pi = Float::with_val(PRECISION, &pi_over_4 * 4i64);

        if i % 1_000_000 == 0 { println!("i: {}, pi: {}", i, pi); }
    }
}
