use std::time::Instant;

fn run_benchmark() {
    println!("Rust Benchmark");
    
    // Test 1: Array operations (extremely optimized)
    let start_time = Instant::now();
    
    // Create a fixed-size array on stack (no heap allocation)
    const SIZE: usize = 1_000_000;
    let mut buf = [0u32; SIZE];
    
    // Use SIMD-friendly sequential access pattern
    for i in 0..SIZE {
        buf[i] = i as u32;
    }
    
    // Compute sum using a linear scan with manual unrolling for better instruction pipelining
    let mut sum: u64 = 0;
    let mut i = 0;
    
    // Process 8 elements at a time
    while i + 8 <= SIZE {
        sum += buf[i] as u64;
        sum += buf[i+1] as u64;
        sum += buf[i+2] as u64;
        sum += buf[i+3] as u64;
        sum += buf[i+4] as u64;
        sum += buf[i+5] as u64;
        sum += buf[i+6] as u64;
        sum += buf[i+7] as u64;
        i += 8;
    }
    
    // Process any remaining elements
    while i < SIZE {
        sum += buf[i] as u64;
        i += 1;
    }
    
    // Prevent optimizer from removing the computation
    if sum == 0 {
        println!("This will never print: {}", sum);
    }
    
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