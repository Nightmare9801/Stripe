use criterion::{criterion_group, criterion_main, Criterion};

pub(crate) fn approximate() -> f64 {
    let mut e: f64 = 1.0;
    let mut factorial: f64 = 1.0;
    for i in 1..28 {
        //print!("{}\r", e);
        //println!("{}", 1.0 / (factorial(i as u128) as f64));
        //print!("Iteration: {}\r", i + 1);
        factorial *= (28 - i) as f64;
        e += factorial;
    }
    e /= factorial;
    e
}


fn bench_approx(c: &mut Criterion) {
    c.bench_function("Approximater Benchmark", |b| b.iter(|| approximate()));
}

// Create a benchmark group
criterion_group!(benches, bench_approx);

// Run the benchmarks
criterion_main!(benches);