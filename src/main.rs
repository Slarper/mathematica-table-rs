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

    // Simplest Situation: table![expr]
    [$e:expr] => {{
        $e
    }};
    // Allow trailing comma
    [$e:expr,] => {{
        $e
    }};

    // Normal Situation: table![expr, {x, iter}]
    [$e:expr, {$x:ident, $i:expr}] => {{
        let mut v = vec![];
        for $x in $i {
            v.push(table![$e])
        };
        v
    }};
    [$e:expr, {$x:ident, $i:expr} $($j:tt)*] => {{
        table![table![$e $($j)*], {$x, $i}]
    }};

    // Special Situation: table![expr, {_, iter}]
    // Ignore the index value
    [$e:expr, {_, $i:expr}] => {{
        let mut v = vec![];
        for _ in $i {
            v.push(table![$e])
        };
        v
    }};
    [$e:expr, {_, $i:expr} $($j:tt)*] => {{
        table![table![$e $($j)*], $i]
    }};

    // Simplified Situation: table![expr, iter]
    // Only iterator.
    [$e:expr, $i:expr] => {{
        table![$e, {_, $i}]
    }};
    [$e:expr, $i:expr, $($j:tt)*] => {{
        table![table![$e,$($j)*], $i]
    }};
}

fn main() {

    let a = table![
        {
            let b = x + 1;
            x + b
        }
        , {x, 0..2},
    ];
    println!("{:?}", a);

}