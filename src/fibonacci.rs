pub fn generate(n: usize) -> Vec<u128> {
    let mut fib: Vec<u128> = Vec::from([0, 1]);

    if n < 2 {
        return fib;
    }

    for i in 1..n {
        fib.push(fib[i] + fib[i - 1]);
    }

    fib
}

pub fn generate_recursive(n: u32) -> u128 {
    if n < 2 {
        return u128::from(n);
    }

    return generate_recursive(n - 1) + generate_recursive(n - 2);
}
