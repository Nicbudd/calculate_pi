
use rug::{
    Integer,
    float::{Round},
    Float,
    ops::{AddAssignRound, MulAssignRound, DivAssignRound, Pow}
};

fn main() {
    ramanujan();
}

fn ramanujan() {
    const RAMANUJAN_PRECISION: u32 = 1000;
    let mut sum = Float::new(RAMANUJAN_PRECISION);

    for k in 0..100u32 {

        let sqrt_two = Float::with_val(RAMANUJAN_PRECISION, 2i32).root(2);
        let a = Integer::from(Integer::factorial(4 * &k));
        let b = Integer::from(1103 + 26390*&k);
        let c = Integer::from(Integer::factorial(1 * &k)).pow(4);
        let d = Integer::from(Integer::from(396u32).pow(4 * &k));

        let mut e = Float::with_val(RAMANUJAN_PRECISION, 1u32);
        e.mul_assign_round(Integer::from(&a * &b), Round::Up);
        e.div_assign_round(Integer::from(&c * &d), Round::Up);

        //println!("a: {}, b: {}, c: {}, d: {}, e: {}", a, b, c, d, e);

        sum.add_assign_round(e, Round::Up);

        let one_over_pi = Float::with_val(RAMANUJAN_PRECISION,
            &sum * Float::with_val(RAMANUJAN_PRECISION,
                Float::with_val(RAMANUJAN_PRECISION, 2u64 * &sqrt_two)
                / (9801u64)
            )
        );

        let pi = Float::with_val(RAMANUJAN_PRECISION, 1.0f64 / one_over_pi);
        println!("{}", pi);
    }
}

#[allow(dead_code)]
fn leibniz() {
    const LEIBNIZ_PRECISION: u32 = 100;
    let mut pi_over_4 = Float::with_val(LEIBNIZ_PRECISION, 1.0);

    for i in 0..10000000000u64 {

        let mut denominator = ((i << 1) + 3) as f64;
        if i % 2 == 0 { denominator *= -1.0; }

        let fraction = Float::with_val(LEIBNIZ_PRECISION, 1.0 / denominator);
        //println!("frac:{}", fraction);
        pi_over_4.add_assign_round(fraction, Round::Up);

        let pi = Float::with_val(LEIBNIZ_PRECISION, &pi_over_4 * 4i64);

        if i % 1_000_000 == 0 { println!("i: {}, pi: {}", i, pi); }
    }
}
