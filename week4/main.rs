use std::time::Instant;

const A: u32 = 1664525;
const C: u32 = 1013904223;

struct Lcg {
    state: u32,
}

impl Lcg {
    #[inline(always)]
    fn new(seed: u32) -> Self {
        Lcg { state: seed }
    }

    #[inline(always)]
    fn next(&mut self) -> u32 {
        // Wrapping arithmetic gives modulo 2^32 automatically.
        self.state = self.state.wrapping_mul(A).wrapping_add(C);
        self.state
    }
}

#[inline(always)]
fn max_subarray_sum(n: usize, seed: u32, min_val: i64, max_val: i64) -> i64 {
    let mut rng = Lcg::new(seed);
    let range = (max_val - min_val + 1) as u64;

    let mut max_ending_here: i64 = 0;
    let mut max_sofar: i64 = i64::MIN;

    for _ in 0..n {
        let rnd = rng.next() as u64;
        let val = (rnd % range) as i64 + min_val;

        if max_ending_here > 0 {
            max_ending_here += val;
        } else {
            max_ending_here = val;
        }

        if max_ending_here > max_sofar {
            max_sofar = max_ending_here;
        }
    }

    max_sofar
}

fn total_max_subarray_sum(n: usize, initial_seed: u32, min_val: i64, max_val: i64) -> i64 {
    let mut seed_gen = Lcg::new(initial_seed);
    let mut total_sum: i64 = 0;

    for _ in 0..20 {
        let seed = seed_gen.next();
        total_sum += max_subarray_sum(n, seed, min_val, max_val);
    }

    total_sum
}

fn main() {
    // Parameters
    let n: usize = 10000;
    let initial_seed: u32 = 42;
    let min_val: i64 = -10;
    let max_val: i64 = 10;

    let start = Instant::now();
    let result = total_max_subarray_sum(n, initial_seed, min_val, max_val);
    let elapsed = start.elapsed();

    println!("Total Maximum Subarray Sum (20 runs): {}", result);
    println!("Execution Time: {:.6} seconds", elapsed.as