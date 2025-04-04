// `find_min!` will calculate the minimum of any number of arguments.
macro_rules! find_min {
    // Base case:
    ($x:expr) => ($x);
    // `$x` followed by at least one `$y,`
    ($x:expr, $($y:expr),+) => (
        // Call `find_min!` on the tail `$y`
        // {// don't work
        //     println!("{}", $x);
        //     return std::cmp::min($x, find_min!($($y),+));
        // }
        {// work
            println!("{}", $x);
            std::cmp::min($x, find_min!($($y),+))
        }
        // { std::cmp::min($x, find_min!($($y),+)) }
    )
}

fn main() {
    println!("{}", find_min!(1u32));
    println!("{}", find_min!(1u32 + 2, 2u32));
    println!("{}", find_min!(5u32, 2u32 * 3, 4u32));
}
