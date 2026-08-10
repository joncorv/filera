fn main() {
    // Run registered benchmarks.
    divan::main();
}

// Register a `fibonacci` function and benchmark it over multiple cases.
#[divan::bench(args = [1, 2, 4, 8, 16, 32])]
fn fibonacci(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        fibonacci(n - 2) + fibonacci(n - 1)
    }
}

#[divan::bench(args = [10, 20, 40, 80, 160, 320])]
fn simple_test(n: usize) {
    let my_struct = 0..n;

    for num in my_struct {
        let nothing = num.clone();
        if nothing % 2 == 0 {
            //
        }
    }
}
