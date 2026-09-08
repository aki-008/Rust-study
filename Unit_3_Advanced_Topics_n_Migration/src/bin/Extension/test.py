import Extension

# Call Rust function
result = Extension.fibonacci(50)
print(result)  # 12586269025 — computed in microseconds

# Use Rust class
counter = Extension.Counter(0)
counter.increment()
counter.increment()
print(counter.get_value())  # 2
print(counter)              # Counter(value=2)

# Performance comparison:
import time

# Python version
def py_primes(n):
    sieve = [True] * (n + 1)
    for i in range(2, int(n**0.5) + 1):
        if sieve[i]:
            for j in range(i*i, n+1, i):
                sieve[j] = False
    return [i for i in range(2, n+1) if sieve[i]]

start = time.perf_counter()
py_result = py_primes(10_000_000)
py_time = time.perf_counter() - start

start = time.perf_counter()
rs_result = Extension.primes_up_to(10_000_000)
rs_time = time.perf_counter() - start

print(f"Python: {py_time:.3f}s")
print(f"Rust:   {rs_time:.3f}s")
print(f"Same results: {py_result == rs_result}")