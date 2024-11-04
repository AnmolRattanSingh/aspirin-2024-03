# Benchmark Results

This is the performance of the `merge_k_arrays_parallel` function with varying numbers of threads, using a large dataset of 10 million elements split into 100 sorted subarrays.

| Threads | Time (seconds) |
|---------|----------------|
| 1       | 0.6305262      |
| 2       | 0.39317653      |
| 4       | 0.29554126      |
| 8       | 0.2726559      |
| 16       | 0.26997575      |
| 32       | 0.2758937      |
| 64       | 0.27392676      |
| 100       | 0.2774164      |

## Analysis

- **Optimal Number of Threads**: In this case, **16 threads** yielded the fastest runtime.
- **Performance Plateau**: Performance improvements decreased after 8–16 threads. Beyond this point, the overhead from managing additional threads outweighed the parallelism benefits.

## Pros and Cons of Different Thread Counts

### More Threads

**Pros**:

- **Faster Completion on Multi-Core Systems**: With enough cores, parallel tasks can be completed more quickly.

**Cons**:

- **Diminishing Returns**: After a point, adding more threads yields limited benefits, due to scheduling overhead.
- **Increased Overhead**: More threads mean more context switching, which can reduce overall efficiency.

### Fewer Threads

**Pros**:

- **Lower Overhead**: Fewer threads mean lower scheduling overhead.
- **Simplicity**: Managing fewer threads reduces complexity and can help prevent concurrency issues.

**Cons**:

- **Underutilization of Resources**: Not fully utilizing all available CPU cores.
- **Longer Execution Time**: Less parallelism can result in longer execution times for larger datasets.
