
use std::mem;

pub fn fibonacci(n: u64) -> u64 {
    fn next(a: &mut u64, b: &mut u64) {
        mem::swap(a, b);
        *b += *a;
    }
    match n {
        0 => 1,
        1 => 1,
        n => {
            let mut a:u64 = 1;
            let mut b = 1;
            for _ in 2..=n {
                next(&mut a, &mut b);
            }
            return b;
        },
    }
}

pub fn fibonacci_cache(n: u64) -> u64 {
    static mut CACHE: Vec<u64> = vec![];

    match n {
        0 => 1,
        1 => 1,
        n => unsafe {
            let ind:usize = (n - 2) as usize;
            if ind >= CACHE.len()  {
                let r = fibonacci_cache(n-1) + fibonacci_cache(n - 2);
                CACHE.push(r);
                r
            } else {
                CACHE[ind]
            }
        },
    }
}

pub fn empty_instruction(){}
