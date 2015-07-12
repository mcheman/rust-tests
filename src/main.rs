mod guess;
mod sieve;

fn main() {
    //guess::guess_number(10000, 10);
    
    let primes = sieve::find_primes(1001);
    println!("{}", primes);
}
