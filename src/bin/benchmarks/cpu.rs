use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main() {
    println!("Running CPU benchmark");

    let results: Vec<u128> = (1..30_000_000) 
        .into_par_iter()
        .map(|val| cpu_intensive_task(val % 20 + 1)) 
        .collect(); 
    // collect into a vec to store this somewhere, will 
    // optimize to nothing otherwise

    do_something(&results);
    
    println!("Finished running CPU benchmark");
}

// no compiler optimization for u
fn do_something(results: &[u128]) {
    // just sum all the values 
    let sum: u128 = results.iter().sum();
    
    println!("Don't optimize me out: {}", sum);
}

fn cpu_intensive_task(num: u128) -> u128 {
    let mut result: u128 = 1;

    for _ in 0..500 { 
        // i cant get consistent results with stuff that overflows
        // so we just use the wrapping versions of the operators
        // and do usually expensive computation
        let big_number = 123456789;
        result = (result.wrapping_mul(num + 1)).wrapping_add(big_number);
        result = result.wrapping_pow(3); 
    }
    result
}
