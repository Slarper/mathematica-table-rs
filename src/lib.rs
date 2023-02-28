/// **Mathematica Table function**
///
/// ```mathematica
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
/// ```console
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
///     {0..3} // omit the index
/// ];
/// println!("{:?}", a);
/// ```
///
/// Output:
/// ```console
/// [["0", "0", "0"], ["1", "1", "1"]]
/// ```
///
/// **Allows trailing comma.**
///
/// Example:
/// ```
/// table![1, {0..3},]
/// ```
///
/// **CAN NOT BE `const`**
///
/// **Return Type is a nested Vec.**
#[macro_export]
macro_rules! table {

    // table![expr]
    [$e:expr $(,)?] => {{
        $e
    }};

    // Normal:
    // table![expr, {x, iter}]
    [$e:expr, {$x:ident, $i:expr}] => {{
        let mut v = Vec::new();
        for $x in $i {
            v.push($e)
        };
        v
    }};

    // Allow ignoring the index value:
    // table![expr, {_, iter}]
    [$e:expr, {_, $i:expr}] => {{
        vec![$e; Iterator::count(IntoIterator::into_iter($i))]
    }};

    // table![expr, {iter}]
    [$e:expr, {$i:expr}] => {{
        table![$e, {_, $i}]
    }};

    // Apply listably
    [$e:expr, {$($fst:tt)*}, $($otr:tt)*] => {{
        table![table![$e, $($otr)*], {$($fst)*}]
    }};

}

/// Less Flexible but
///
/// **CAN BE `const`**
///
/// ```
/// const ARR : [i32;3] = array![x,{x,2,4,1}]; // [2 3 4]
/// ```
#[macro_export]
macro_rules! array {

    [$e:expr $(,)?] => {{
        $e
    }};

    // Normal:
    // array![expr, {x, st,end, dlt}]
    [$e:expr, {$x:ident, $st:expr, $end:expr, $dlt:expr}] => {{

        // Calculate Length of Array
        const LEN: usize = (($end - $st) / $dlt) as usize + 1;

        // Loop Initialization.
        // Can use unsafe for better code.
        let mut $x = $st;
        let mut i = 0;
        let mut arr = [$e; LEN];
        i += 1;
        $x = $x + $dlt;

        if ($dlt as f64) > 0f64
        {
            while $x <= $end
            {
                arr[i] = $e;
                $x = $x + $dlt;
                i += 1;
            }
        } else {
            while $x >= $end
            {
                arr[i] = $e;
                $x = $x + $dlt;
                i += 1;
            }
        }

        arr
    }};

    // array![expr, {x, start, end}]
    [$e:expr, {$x:ident, $st:expr, $end:expr} ] => {{
        array![$e, {$x, $st, $end, 1}]
    }};

    // array![expr, {x, end}]
    [$e:expr, {$x:ident, $end:expr} ] => {{
        array![$e, {$x, 1, $end}]
    }};

    // Allow ignoring index:
    // array![expr, {_, st, end, dlt}]
    [$e:expr, {_, $st:expr, $end:expr, $dlt:expr}] => {{

        // Calculate Length of Array
        const LEN: usize = (($end - $st) / $dlt) as usize + 1;

        let arr = [$e; LEN];
        arr
    }};

    // array![expr, {_, st, end}]
    [$e:expr, {_, $st:expr, $end:expr}] => {{
        array![$e, {_, $st, $end, 1}]
    }};

    // array![expr, {_, end}]
    [$e:expr, {_, $end:expr}] => {{
        array![$e, {_,1, $end}]
    }};

    // array![expr, {count}]
    [$e:expr, {$end:expr}] => {{
        array![$e, {_, $end}]
    }};

    // Apply listably
    [$e:expr, {$($fst:tt)*}, $($otr:tt)*] => {{
        array![array![$e, $($otr)*], {$($fst)*}]
    }};

}
#[macro_export]
macro_rules! fold {
    // For functions
    [$f:ident, $e:expr, {}] => {{
        $e
    }};
    [$f:ident, $e:expr, {$fst:expr }] => {{
        $f($e, $fst)
    }};
    [$f:ident, $e:expr, {$fst:expr, $($otr:tt)*  }] => {{
        fold![$f, fold![$f, $e,{$fst}], {$($otr)*}]
    }};

    // For macros
    [$f:ident !, $e:expr, {}] => {{
        $e
    }};
    // expr
    [$f:ident !, $e:expr, {$fst:expr }] => {{
        $f!($e, $fst)
    }};
    [$f:ident !, $e:expr, {$fst:expr, $($otr:tt)*  }] => {{
        fold![$f!, fold![$f!, $e,{$fst}], {$($otr)*}]
    }};
}
