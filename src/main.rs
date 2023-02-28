use hellors::{fold, roll};
// For test of fold!
macro_rules! add {

    ($a:expr, $b:expr ) => {{
        println!("{}", $a + $b);
        $a + $b
    }};

}

fn main() {

    fold!(add!,1,{2, 3, 4});

}