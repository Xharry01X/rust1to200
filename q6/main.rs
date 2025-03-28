// prime number or not

fn main() {

    print_primes_in_range(2,100);
}

fn is_prime(n:u32) -> bool {
    if n <= 1 {
        return false;
    }

    for i in 2..=(n as f64).sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn print_primes_in_range(start: u32, end: u32){
    println!("prime numbers between {} and {}:", start, end);

    for num in start..=end {
        if is_prime(num){
            println!("{}", num);
        }
    }
    println!();
}