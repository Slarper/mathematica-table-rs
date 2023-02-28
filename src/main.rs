use hellors::{ fold};

// For test of fold!
macro_rules! add {
    ($a:expr, $b:expr) => {{
        $a + $b + 1
    }};
}

fn main() {
    let add = |a: i32, b: i32|->i32{a + b * 2};

    let x:i32 = fold![add!, 1, {2}];
    println!("{}", x);

    let y:i32 = fold![add, 1, {2}];
    println!("{}", y);

}