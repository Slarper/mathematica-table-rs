use hellors::{fibonacci, fibonacci_cache};

fn main() {
    for i in 0..=11 {
        println!("{}", fibonacci_cache(i));
    }

    for i in 0..=11 {
        println!("{}", fibonacci(i));
    }
}