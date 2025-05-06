// Node.js version of the benchmark
const { performance } = require('perf_hooks');

function runBenchmark() {
    console.log("JavaScript (Node.js) Benchmark");
    
    // Test 1: Array operations
    const arrayStart = performance.now();
    const arr = [];
    for (let i = 0; i < 1000000; i++) {
        arr.push(i);
    }
    const sum = arr.reduce((acc, val) => acc + val, 0);
    const arrayTime = performance.now() - arrayStart;
    console.log(`Array operations: ${arrayTime.toFixed(2)} ms`);
    
    // Test 2: String operations
    const stringStart = performance.now();
    let str = "";
    for (let i = 0; i < 100000; i++) {
        str += "a";
    }
    const stringTime = performance.now() - stringStart;
    console.log(`String operations: ${stringTime.toFixed(2)} ms`);
    
    // Test 3: Math operations
    const mathStart = performance.now();
    let result = 0;
    for (let i = 0; i < 10000000; i++) {
        const x = i % 1000;
        result += Math.sqrt(x) * Math.sin(x);
        result = result % 1000;
    }
    const mathTime = performance.now() - mathStart;
    console.log(`Math operations: ${mathTime.toFixed(2)} ms`);
    
    // Total time
    const totalTime = arrayTime + stringTime + mathTime;
    console.log(`Total execution time: ${totalTime.toFixed(2)} ms`);
}

// Run the benchmark
runBenchmark(); 