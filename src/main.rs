/// **Mathematica Table function**
///
/// ```
/// Table[expr, iter1, iter2, ...]
/// ```
///
/// For example:
///
/// ```
/// let a = table![
///     {
///         format!("{} x {} = {}", x, y, x * y)
///     },
///     {x, 1..=2},
///     {y, 1..=x}
/// ];
/// println!("{:?}", a);
/// ```
///
/// Output:
/// ```
/// [["1 x 1 = 1"], ["2 x 1 = 2", "2 x 2 = 4"]]
///
/// ```
///
/// **If the loop index is not used, it can be omitted.**
///
/// Example:
/// ```
/// let a = table![
///     {
///         format!("{}", x)
///     },
///     {x, 0..2},
///     0..3 // omit the index
/// ];
/// println!("{:?}", a);
/// ```
///
/// Output:
/// ```
/// [["0", "0", "0"], ["1", "1", "1"]]
/// ```
///
/// **Allows trailing comma.**
///
/// Example:
/// ```
/// table![1, 0..3,]
/// ```
macro_rules! table {
    // Note: $(,)? allows trailing comma.

    // Simplest situation: table![expr] => expr
    [$e:expr $(,)?] => {
        $e
    };

    // Deal with the Iterator Expr with the symbol: table![expr, {x, iter}, ...]
    [$e:expr ,{$x:ident, $i:expr} $(,$j:expr)* $(,)?] => {{
        let mut v = vec![];
        for $x in $i {
            v.push(table![$e $(,$j)*])
        };
        v
    }};

    // Deal with the Iterator Expr without the symbol: table![expr, iter, ...]
    [$e:expr ,$i:expr $(,$j:expr)* $(,)?] => {{
        let mut v = vec![];
        for _ in $i {
            v.push(table![$e $(,$j)*])
        };
        v
    }};
}

fn main() {

    let a = table![format!("{}",x), {x,0..3}, 0..2];
    println!("{:?}", a);

}