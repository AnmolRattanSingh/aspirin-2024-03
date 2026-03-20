use anyhow::Result;
use rand::Rng;
use std::time::Instant;

mod error;
mod thread_pool;
use thread_pool::ThreadPool;

// Merge two sorted arrays into a new sorted array
fn merge_arrays(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut c = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0, 0);

    while i < a.len() && j < b.len() {
        if a[i] < b[j] {
            c.push(a[i]);
            i += 1;
        } else {
            c.push(b[j]);
            j += 1;
        }
    }

    c.extend_from_slice(&a[i..]);
    c.extend_from_slice(&b[j..]);

    c
}

fn merge_k_arrays_parallel(mut arrays: Vec<Vec<i32>>, num_threads: usize) -> Vec<i32> {
    while arrays.len() > 1 {
        let mut thread_pool = ThreadPool::new(num_threads).unwrap();
        let mut merged_arrays = Vec::new();
        let mut i = 0;

        while i < arrays.len() {
            if i + 1 < arrays.len() {
                let a = arrays[i].clone();
                let b = arrays[i + 1].clone();

                // Submit the merge task to the thread pool
                thread_pool.execute(move || merge_arrays(&a, &b)).unwrap();
            } else {
                // Only one array left, add it directly
                merged_arrays.push(arrays[i].clone());
            }
            i += 2;
        }

        // Close the thread pool and collect results
        thread_pool.close();
        let mut results = thread_pool.get_results();
        merged_arrays.append(&mut results);
        arrays = merged_arrays;
    }

    arrays.pop().unwrap()
}

// Generate a large vector of sorted subarrays
fn generate_sorted_subarrays(total_size: usize, num_arrays: usize) -> Vec<Vec<i32>> {
    let mut rng = rand::thread_rng();
    let mut arrays = Vec::with_capacity(num_arrays);

    for _ in 0..num_arrays {
        let mut array: Vec<i32> = (0..total_size / num_arrays)
            .map(|_| rng.gen_range(0..1_000_000))
            .collect();
        array.sort_unstable();
        arrays.push(array);
    }

    arrays
}

fn main() -> Result<()> {
    let total_size = 10_000_000;
    let num_arrays = 100;

    // Generate the sorted subarrays
    let arrays = generate_sorted_subarrays(total_size, num_arrays);

    // Range of thread counts to test
    let thread_counts = [1, 2, 4, 8, 16, 32, 64, 100];

    println!("| Threads | Time (seconds) |");
    println!("|---------|----------------|");

    for &num_threads in &thread_counts {
        let arrays_clone = arrays.clone(); // Clone arrays for each separate run

        // Measure execution time
        let start_time = Instant::now();
        let result = merge_k_arrays_parallel(arrays_clone, num_threads);
        let duration = start_time.elapsed();

        // Print results in a jank table
        println!(
            "| {}       | {}      |",
            num_threads,
            duration.as_secs_f32()
        );

        // Verify result length
        assert_eq!(result.len(), total_size);
    }

    Ok(())
}
