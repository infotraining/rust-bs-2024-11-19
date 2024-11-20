fn is_prime(n: u64) -> bool {
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

fn n_primes(count: usize) -> Vec<u64> {
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

fn primes_in_range(range: std::ops::Range<u64>) -> Vec<u64> {
    let mut result: Vec<u64> = Vec::new();

    for value in range {
        if is_prime(value) {
            result.push(value);
        }
    }

    return result;
}



fn primes_from_vec(range: Vec<u64>) -> Vec<u64> {
    let mut primes = Vec::new();

    for num in range {
        if is_prime(num) {
            primes.push(num);
        }
    }

    primes
}

#[test]
fn test_is_prime() {
    assert!(!is_prime(0));
    assert!(!is_prime(1));
    assert!(is_prime(2));
    assert!(is_prime(3));
    assert!(is_prime(5));
    assert!(is_prime(7));
    assert!(is_prime(11));
    assert!(is_prime(13));
    assert!(is_prime(17));
    assert!(is_prime(19));
    assert!(!is_prime(4));
    assert!(!is_prime(6));
    assert!(!is_prime(8));
    assert!(!is_prime(9));
    assert!(!is_prime(10));
    assert!(is_prime(113));
}

#[test]
fn test_n_primes() {
    let primes = n_primes(0);
    assert_eq!(primes, Vec::new());

    let primes = n_primes(10);
    assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);

    let primes = n_primes(20);
    assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71]);
}

#[test]
fn test_primes_in_range() {
    let primes = primes_in_range(2..100); // std::ops::Range<u32> is iterable
    
    assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 
                            31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 
                            73, 79, 83, 89, 97]);
}

// EXTRA - Iterators
// #[test]
// fn test_all_primes() {
//     // let primes = all_primes().take(10).collect::<Vec<u32>>();
//     // assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
// }

fn main() {
    let  args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);

    for args in &args[1..] {
        let num = args.parse::<u64>().expect("Invalid number");
        if is_prime(num) {
            println!("{} is prime", num);
        } else {
            println!("{} is not prime", num);
        }
    }
}
