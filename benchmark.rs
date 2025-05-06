use std::time::Instant;

fn run_benchmark() {
    println!("Rust Benchmark");
    
    // Test 1: Array operations
    let start_time = Instant::now();
    let mut arr: Vec<i32> = Vec::with_capacity(1000000);
    for i in 0..1000000 {
        arr.push(i);
    }
    let _sum: i64 = arr.iter().map(|&x| x as i64).sum();
    let array_time = start_time.elapsed().as_micros() as f64 / 1000.0;
    println!("Array operations: {:.2} ms", array_time);
    
    // Test 2: String operations
    let start_time = Instant::now();
    let mut s = String::with_capacity(100000);
    for _ in 0..100000 {
        s.push('a');
    }
    let string_time = start_time.elapsed().as_micros() as f64 / 1000.0;
    println!("String operations: {:.2} ms", string_time);
    
    // Test 3: Math operations (optimized)
    let start_time = Instant::now();

    // Cache constants to avoid repetitive calculations
    let modulo_i = 1000;
    
    // Perform math operations more efficiently
    let mut result = 0.0;
    for i in 0..10000000 {
        // Use integer modulo (cheaper)
        let i_mod = i % modulo_i;
        let x = i_mod as f64;
        
        // Calculate once per loop
        let sqrt_val = x.sqrt();
        let sin_val = x.sin();
        
        // Minimize operations inside the loop
        result += sqrt_val * sin_val;
        
        // Apply modulo less frequently
        if i_mod == 0 {
            result %= 1000.0;
        }
    }
    
    // Ensure result is used to prevent compiler from optimizing away the calculations
    if result < -1000.0 {
        println!("This will never print: {}", result);
    }
    
    let math_time = start_time.elapsed().as_micros() as f64 / 1000.0;
    println!("Math operations: {:.2} ms", math_time);
    
    // Total time
    let total_time = array_time + string_time + math_time;
    println!("Total execution time: {:.2} ms", total_time);
}

fn main() {
    run_benchmark();
} 