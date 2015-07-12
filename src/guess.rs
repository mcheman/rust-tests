extern crate rand;

use self::rand::Rng;
use std::io::{self, BufRead};

pub fn guess_number(max_number: u32, max_tries: u32){
  	println!("Guess the number! {} to {}", 0, max_number);

  	let mut rng = rand::thread_rng();
  	let number = rng.gen_range::<u32>(0, max_number);
  
  	let stdin = io::stdin();
  	let mut tries = max_tries;
  	for line in stdin.lock().lines() {
        tries -= 1;
        let guess:u32 = line.unwrap().trim().parse::<u32>().unwrap();
        if guess == number {
        	println!("YAY!\t Done in {} tries", max_tries - tries); 
        	break;
        } else if guess < number {
        	println!("Too Low!\tOnly - {} tries remaining...", tries);
        } else {
        	println!("Too High!\tOnly - {} tries remaining...", tries);
        }

        if tries <= 0 {
        	println!("TOO BAD. Number was {}", number);
        	break;
        }
    }
}