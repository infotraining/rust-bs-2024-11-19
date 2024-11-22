use primes_lib::math_utils::is_prime;


// EXTRA - Iterators
// #[test]
// fn test_all_primes() {
//     // let primes = all_primes().take(10).collect::<Vec<u32>>();
//     // assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
// }

fn main() {
    let args: Vec<String> = std::env::args().collect();
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
