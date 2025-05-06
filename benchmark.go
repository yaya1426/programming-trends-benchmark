package main

import (
	"fmt"
	"math"
	"strings"
	"time"
)

func runBenchmark() {
	fmt.Println("Go Benchmark")

	// Test 1: Array operations
	startTime := time.Now()
	arr := make([]int, 0, 1000000)
	for i := 0; i < 1000000; i++ {
		arr = append(arr, i)
	}
	sum := 0
	for _, v := range arr {
		sum += v
	}
	arrayTime := float64(time.Since(startTime).Microseconds()) / 1000.0
	fmt.Printf("Array operations: %.2f ms\n", arrayTime)

	// Test 2: String operations (Optimized with strings.Builder)
	startTime = time.Now()
	var sb strings.Builder
	sb.Grow(100000) // Pre-allocate capacity
	for i := 0; i < 100000; i++ {
		sb.WriteByte('a')
	}
	// Final string is in sb but we don't need to retrieve it for the benchmark
	stringTime := float64(time.Since(startTime).Microseconds()) / 1000.0
	fmt.Printf("String operations: %.2f ms\n", stringTime)

	// Test 3: Math operations
	startTime = time.Now()
	result := 0.0
	for i := 0; i < 10000000; i++ {
		x := float64(i % 1000)
		result += math.Sqrt(x) * math.Sin(x)
		result = math.Mod(result, 1000.0)
	}
	mathTime := float64(time.Since(startTime).Microseconds()) / 1000.0
	fmt.Printf("Math operations: %.2f ms\n", mathTime)

	// Total time
	totalTime := arrayTime + stringTime + mathTime
	fmt.Printf("Total execution time: %.2f ms\n", totalTime)
}

func main() {
	runBenchmark()
}
