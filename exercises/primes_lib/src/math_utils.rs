pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }

    for i in 2..=(n as f64).sqrt() as u64 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn n_primes(count: usize) -> Vec<u64> {
    let mut result: Vec<u64> = Vec::new();
    let mut candidate: u64 = 2;

    while result.len() < count {
        if is_prime(candidate) {
            result.push(candidate);
        }
        candidate += 1;
    }

    result
}

pub fn primes_in_range(range: std::ops::Range<u64>) -> Vec<u64> {
    let mut result: Vec<u64> = Vec::new();

    for value in range {
        if is_prime(value) {
            result.push(value);
        }
    }

    return result;
}

pub fn primes_from_vec(range: Vec<u64>) -> Vec<u64> {
    let mut primes = Vec::new();

    for num in range {
        if is_prime(num) {
            primes.push(num);
        }
    }

    primes
}