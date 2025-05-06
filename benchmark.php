<?php

function runBenchmark() {
    echo "PHP Benchmark\n";
    
    // Test 1: Array operations
    $startTime = microtime(true);
    $arr = [];
    for ($i = 0; $i < 1000000; $i++) {
        $arr[] = $i;
    }
    $sumArr = array_sum($arr);
    $endTime = microtime(true);
    $arrayTime = ($endTime - $startTime) * 1000;
    echo "Array operations: " . number_format($arrayTime, 2) . " ms\n";
    
    // Test 2: String operations
    $startTime = microtime(true);
    $str = "";
    for ($i = 0; $i < 100000; $i++) {
        $str .= "a";
    }
    $endTime = microtime(true);
    $stringTime = ($endTime - $startTime) * 1000;
    echo "String operations: " . number_format($stringTime, 2) . " ms\n";
    
    // Test 3: Math operations
    $startTime = microtime(true);
    $result = 0;
    for ($i = 0; $i < 10000000; $i++) {
        $x = $i % 1000;
        $result += sqrt($x) * sin($x);
        $result = fmod($result, 1000);
    }
    $endTime = microtime(true);
    $mathTime = ($endTime - $startTime) * 1000;
    echo "Math operations: " . number_format($mathTime, 2) . " ms\n";
    
    // Total time
    $totalTime = $arrayTime + $stringTime + $mathTime;
    echo "Total execution time: " . number_format($totalTime, 2) . " ms\n";
}

// Run the benchmark
runBenchmark(); 