# Programming Language Performance Comparison

This repository contains benchmark files for PHP, Go, Rust, and Node.js to compare their performance.

## Benchmark Tests

Each benchmark file performs three tests with identical methodology:
1. **Array operations**: Creates and processes an array of 1,000,000 elements
2. **String operations**: Concatenates 100,000 characters
3. **Math operations**: Performs 10,000,000 calculations using square root and sine functions with modulo to prevent overflow

## Running the Benchmarks

### PHP
```bash
php benchmark.php
```

### Go
```bash
go run benchmark.go
```

### Rust
```bash
# For best performance, compile with optimization
rustc -C opt-level=3 benchmark.rs
./benchmark
```

### Node.js
```bash
node benchmark_node.js
```

## Files Included

- `benchmark.php` - PHP benchmark
- `benchmark.go` - Go benchmark (using strings.Builder for efficient string operations)
- `benchmark.rs` - Rust benchmark (optimized for both array and math operations)
- `benchmark_node.js` - JavaScript benchmark for Node.js

## Expected Output Format

Each benchmark will output timing results with 2 decimal places in milliseconds for:
- Array operations
- String operations
- Math operations
- Total execution time

## Sample Results

```
# PHP
Array operations: 11.02 ms
String operations: 0.94 ms
Math operations: 465.77 ms
Total execution time: 477.73 ms

# Go
Array operations: 1.49 ms
String operations: 0.06 ms
Math operations: 90.59 ms
Total execution time: 92.13 ms

# Rust (with -C opt-level=3)
Array operations: 0.35 ms
String operations: 0.06 ms
Math operations: 58.34 ms
Total execution time: 58.74 ms

# Node.js
Array operations: 22.51 ms
String operations: 0.99 ms
Math operations: 120.91 ms
Total execution time: 144.42 ms
```

## Key Performance Insights

### Performance By Category

1. **Array Operations**:
   - Rust is the fastest (0.35 ms)
   - Go is second (1.49 ms) - 326% slower than Rust
   - PHP (11.02 ms) - 3,049% slower than Rust
   - Node.js is slowest (22.51 ms) - 6,331% slower than Rust

2. **String Operations**:
   - Go and Rust tie (0.06 ms)
   - PHP (0.94 ms) - 1,467% slower than Go/Rust
   - Node.js (0.99 ms) - 1,550% slower than Go/Rust

3. **Math Operations**:
   - Rust is significantly faster (58.34 ms)
   - Go is second (90.59 ms) - 55% slower than Rust
   - Node.js is third (120.91 ms) - 107% slower than Rust
   - PHP is much slower (465.77 ms) - 698% slower than Rust

### Overall Performance

| Language | Total Time | % Slower than Rust | Comparison |
|----------|------------|-------------------|------------|
| Rust     | 58.74 ms   | Baseline          | Fastest overall |
| Go       | 92.13 ms   | 57% slower        | 1.6× slower than Rust |
| Node.js  | 144.42 ms  | 146% slower       | 2.5× slower than Rust |
| PHP      | 477.73 ms  | 713% slower       | 8.1× slower than Rust |

### Language by Language Comparison

- **Rust vs Go**: Rust is 36% faster overall. Rust has a dramatic edge in array operations (326% faster) and significant advantage in math operations (55% faster), while they tie in string operations.

- **Rust vs Node.js**: Rust is 59% faster overall. The biggest difference is in array operations (Rust is 6,331% faster) and math operations (Rust is 107% faster).

- **Rust vs PHP**: Rust is 88% faster overall, with the most dramatic difference in array operations (3,049% faster) and math operations (698% faster).

- **Go vs Node.js**: Go is 36% faster overall, with significant advantage in array operations (1,411% faster).

- **Go vs PHP**: Go is 81% faster overall, with the biggest gap in math operations (414% faster).

- **Node.js vs PHP**: Node.js is 70% faster overall, primarily due to much better performance in math operations (285% faster).

## Notes for Presentation

When comparing the results:
- Rust excels at computational tasks when properly optimized
- Rust's array operations are over 4x faster than Go when optimized to use stack allocation and loop unrolling
- Go performs very well in string manipulation with proper tools
- Node.js offers balanced performance but lags in array operations
- PHP is notably slower in math operations but better than expected at array handling
- Consider the compilation vs interpretation differences
- The benchmarks test raw performance, not language features or developer productivity
- For compiled languages like Go and Rust, compile-time optimization significantly impacts performance 