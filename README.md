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
- `benchmark.rs` - Rust benchmark (optimized for math operations)
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
Array operations: 1.79 ms
String operations: 0.05 ms
Math operations: 92.45 ms
Total execution time: 94.30 ms

# Rust (with -C opt-level=3)
Array operations: 0.93 ms
String operations: 0.06 ms
Math operations: 57.23 ms
Total execution time: 58.22 ms

# Node.js
Array operations: 22.51 ms
String operations: 0.99 ms
Math operations: 120.91 ms
Total execution time: 144.42 ms
```

## Key Performance Insights

### Performance By Category

1. **Array Operations**:
   - Rust is the fastest (0.93 ms)
   - Go is close second (1.79 ms) - 92% slower than Rust
   - PHP (11.02 ms) - 1,085% slower than Rust
   - Node.js is slowest (22.51 ms) - 2,320% slower than Rust

2. **String Operations**:
   - Go is fastest (0.05 ms)
   - Rust is close (0.06 ms) - 20% slower than Go
   - PHP (0.94 ms) - 1,780% slower than Go
   - Node.js (0.99 ms) - 1,880% slower than Go

3. **Math Operations**:
   - Rust is significantly faster (57.23 ms)
   - Go is second (92.45 ms) - 62% slower than Rust
   - Node.js is third (120.91 ms) - 111% slower than Rust
   - PHP is much slower (465.77 ms) - 714% slower than Rust

### Overall Performance

| Language | Total Time | % Slower than Rust | Comparison |
|----------|------------|-------------------|------------|
| Rust     | 58.22 ms   | Baseline          | Fastest overall |
| Go       | 94.30 ms   | 62% slower        | 1.6× slower than Rust |
| Node.js  | 144.42 ms  | 148% slower       | 2.5× slower than Rust |
| PHP      | 477.73 ms  | 721% slower       | 8.2× slower than Rust |

### Language by Language Comparison

- **Rust vs Go**: Rust is 38% faster overall. Rust excels in array operations (48% faster) and math operations (38% faster), while Go has a slight edge in string operations.

- **Rust vs Node.js**: Rust is 60% faster overall. The biggest difference is in array operations (Rust is 96% faster) and math operations (Rust is 53% faster).

- **Rust vs PHP**: Rust is 88% faster overall, with the most dramatic difference in math operations (Rust is 88% faster).

- **Go vs Node.js**: Go is 35% faster overall, with significant advantage in array operations (92% faster) and string operations (95% faster).

- **Go vs PHP**: Go is 80% faster overall, with the biggest gap in math operations (80% faster).

- **Node.js vs PHP**: Node.js is 70% faster overall, primarily due to much better performance in math operations (74% faster).

## Notes for Presentation

When comparing the results:
- Rust excels at computational tasks when properly optimized
- Go performs very well in array operations and string manipulation with proper tools
- Node.js offers balanced performance but lags in array operations
- PHP is notably slower in math operations but better than expected at array handling
- Consider the compilation vs interpretation differences
- The benchmarks test raw performance, not language features or developer productivity
- For compiled languages like Go and Rust, compile-time optimization significantly impacts performance 