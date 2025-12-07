#!/usr/bin/env python3
"""
Benchmark comparison: Python curl_cffi vs Rust curl-cffi-rs

Requirements:
- pip install curl_cffi pandas matplotlib
- cargo build --release --features python
- Test server running on localhost:8000
"""

import time
import sys
import os
from io import BytesIO
import pandas as pd
import matplotlib.pyplot as plt

# Import Python curl_cffi (from installed package)
import curl_cffi
from curl_cffi import Curl as PythonCurl
from curl_cffi.requests import Session

# Try to import Rust version (if built)
try:
    sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'target', 'release'))
    import hyprcurl as rust_curl
    RUST_AVAILABLE = True
except ImportError:
    RUST_AVAILABLE = False
    print("Warning: Rust hyprcurl not available. Build with: cargo build --release --features python")

results = []

def bench_python_raw(url, iterations=1000):
    """Benchmark Python curl_cffi raw interface"""
    curl = PythonCurl()

    start = time.time()
    for _ in range(iterations):
        buffer = BytesIO()
        curl.setopt(curl_cffi.CurlOpt.URL, url)
        curl.setopt(curl_cffi.CurlOpt.WRITEDATA, buffer)
        curl.perform()
    duration = time.time() - start

    curl.close()
    return duration

def bench_python_session(url, iterations=1000):
    """Benchmark Python curl_cffi Session interface"""
    session = Session()

    start = time.time()
    for _ in range(iterations):
        session.get(url)
    duration = time.time() - start

    session.close()
    return duration

def bench_rust_raw(url, iterations=1000):
    """Benchmark Rust curl-cffi-rs"""
    if not RUST_AVAILABLE:
        return None

    curl = rust_curl.Curl()

    start = time.time()
    for _ in range(iterations):
        curl.set_url(url)
        data = curl.perform()
    duration = time.time() - start

    return duration

def bench_rust_convenience(url, iterations=1000):
    """Benchmark Rust convenience function"""
    if not RUST_AVAILABLE:
        return None

    start = time.time()
    for _ in range(iterations):
        data = rust_curl.get(url)
    duration = time.time() - start

    return duration

def run_benchmarks():
    """Run all benchmarks"""
    print("=" * 60)
    print("Python curl_cffi vs Rust curl-cffi-rs Benchmark")
    print("=" * 60)
    print()

    sizes = ["1k", "20k", "200k"]
    iterations = 1000

    for size in sizes:
        url = f"http://localhost:8000/{size}"
        print(f"\nBenchmarking {size} responses ({iterations} iterations)...")
        print("-" * 60)

        # Python raw
        py_raw_time = bench_python_raw(url, iterations)
        print(f"Python Raw:    {py_raw_time:.3f}s ({iterations/py_raw_time:.1f} req/s)")
        results.append({
            'implementation': 'Python Raw',
            'size': size,
            'duration': py_raw_time,
            'req_per_sec': iterations / py_raw_time
        })

        # Python session
        py_session_time = bench_python_session(url, iterations)
        print(f"Python Session: {py_session_time:.3f}s ({iterations/py_session_time:.1f} req/s)")
        results.append({
            'implementation': 'Python Session',
            'size': size,
            'duration': py_session_time,
            'req_per_sec': iterations / py_session_time
        })

        # Rust raw
        if RUST_AVAILABLE:
            rust_raw_time = bench_rust_raw(url, iterations)
            print(f"Rust Raw:      {rust_raw_time:.3f}s ({iterations/rust_raw_time:.1f} req/s)")
            speedup = py_raw_time / rust_raw_time
            print(f"               ↑ {speedup:.2f}x faster than Python Raw")
            results.append({
                'implementation': 'Rust Raw',
                'size': size,
                'duration': rust_raw_time,
                'req_per_sec': iterations / rust_raw_time
            })

            # Rust convenience
            rust_conv_time = bench_rust_convenience(url, iterations)
            print(f"Rust Get:      {rust_conv_time:.3f}s ({iterations/rust_conv_time:.1f} req/s)")
            results.append({
                'implementation': 'Rust Get',
                'size': size,
                'duration': rust_conv_time,
                'req_per_sec': iterations / rust_conv_time
            })

    print("\n" + "=" * 60)
    print("Benchmark Complete!")
    print("=" * 60)

    # Save results
    df = pd.DataFrame(results)
    df.to_csv('python_vs_rust_bench.csv', index=False)
    print("\nResults saved to: python_vs_rust_bench.csv")

    # Generate plots
    generate_plots(df)

def generate_plots(df):
    """Generate comparison plots"""
    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(14, 6))

    # Plot 1: Duration comparison
    for impl in df['implementation'].unique():
        data = df[df['implementation'] == impl]
        ax1.plot(data['size'], data['duration'], marker='o', label=impl, linewidth=2)

    ax1.set_xlabel('Response Size', fontsize=12)
    ax1.set_ylabel('Time (seconds)', fontsize=12)
    ax1.set_title('Request Duration (1000 iterations)\nLower is Better', fontsize=14, fontweight='bold')
    ax1.legend()
    ax1.grid(True, alpha=0.3)

    # Plot 2: Requests per second
    for impl in df['implementation'].unique():
        data = df[df['implementation'] == impl]
        ax2.plot(data['size'], data['req_per_sec'], marker='s', label=impl, linewidth=2)

    ax2.set_xlabel('Response Size', fontsize=12)
    ax2.set_ylabel('Requests/Second', fontsize=12)
    ax2.set_title('Throughput (req/s)\nHigher is Better', fontsize=14, fontweight='bold')
    ax2.legend()
    ax2.grid(True, alpha=0.3)

    plt.tight_layout()
    plt.savefig('python_vs_rust_bench.png', dpi=150, bbox_inches='tight')
    print("Plot saved to: python_vs_rust_bench.png")

    # Calculate and display speedups
    if RUST_AVAILABLE:
        print("\n" + "=" * 60)
        print("SPEEDUP ANALYSIS")
        print("=" * 60)
        for size in df['size'].unique():
            py_raw = df[(df['implementation'] == 'Python Raw') & (df['size'] == size)]['duration'].values[0]
            rust_raw = df[(df['implementation'] == 'Rust Raw') & (df['size'] == size)]['duration'].values[0]
            speedup = py_raw / rust_raw
            print(f"{size:>5}: Rust is {speedup:.2f}x faster than Python")

if __name__ == '__main__':
    try:
        run_benchmarks()
    except KeyboardInterrupt:
        print("\n\nBenchmark interrupted by user.")
    except Exception as e:
        print(f"\n\nError: {e}")
        print("\nMake sure:")
        print("1. Test server is running: python server.py")
        print("2. Rust library is built: cargo build --release --features python")
        sys.exit(1)
