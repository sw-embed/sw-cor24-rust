//! C pipeline example data — loaded from files via include_str!()

use components::CExample;

fn example(name: &str, description: &str, c_source: &str, cor24_asm: &str) -> CExample {
    CExample {
        name: name.to_string(),
        description: description.to_string(),
        c_source: c_source.to_string(),
        cor24_assembly: cor24_asm.to_string(),
    }
}

pub fn get_c_examples() -> Vec<CExample> {
    vec![
        example(
            "Fibonacci",
            "Recursive fib(10) with printf — prints \"Fibonacci 10\" then \"89\"",
            include_str!("examples/c_pipeline/fib.c"),
            include_str!("examples/c_pipeline/fib.cor24.s"),
        ),
        example(
            "Sieve of Eratosthenes",
            "Compute prime count below 16384 — prints \"1899 primes.\"",
            include_str!("examples/c_pipeline/sieve.c"),
            include_str!("examples/c_pipeline/sieve.cor24.s"),
        ),
    ]
}
