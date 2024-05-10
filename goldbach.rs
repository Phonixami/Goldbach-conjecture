extern crate primal;

use std::env;

fn get_input() -> usize {
    env::args()
        .nth(1)
        .expect("Single positive integer expected as argument")
        .parse::<usize>()
        .expect("Single positive integer expected as argument")
}

fn list_primes_until(input: &usize) -> Vec<usize> {
    let sieve = primal::Sieve::new(input.clone());
    return sieve.primes_from(0).take_while(|x| x <= input).collect();
}

fn get_goldbach_solution(input: &usize) -> Option<[usize; 3]> {
    let first_primes = list_primes_until(&input);

    for first_prime in first_primes {
        if input - first_prime < 1 {
            continue;
        }

        let second_primes = list_primes_until(&(input - first_prime));
        for second_prime in second_primes {
            if input - first_prime - second_prime < 1 {
                continue;
            }

            let third_primes = list_primes_until(&(input - first_prime - second_prime));
            for third_prime in third_primes {
                if &(first_prime + second_prime + third_prime) == input {
                    return Some([first_prime, second_prime, third_prime]);
                }
            }
        }
    }

    return None;
}

fn main() {
    let input = get_input();

    let solution = get_goldbach_solution(&input).expect("No solution found for input");
    println!(
        "{} = {} + {} + {}",
        input,
        solution[0],
        solution[1],
        solution[2]
    );
}