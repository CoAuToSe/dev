#![recursion_limit = "256"]
use std::{
    env::{self, *},
    path::{self, *},
};
macro_rules! show {
    ($a:expr) => {
        println!("{:?}", $a);
};
    ($a:expr, $($b:expr),+) => {
        print!("{}", $a);
        show!($($b),+);
    };
}

fn main() {
    println!("Hello, world!");
    show!("Hello, world!");
    show!(args_os());
    show!(args());
    let paths = [Path::new("/bin"), Path::new("/usr/bin")];
    show!(join_paths(paths.iter()));
    println!();

    let key = "PATH";
    match env::var_os(key) {
        Some(paths) => {
            for path in env::split_paths(&paths) {
                println!("'{}'", path.display());
            }
        }
        None => println!("{} is not defined in the environment.", key),
    }
    println!();
    let key = "HOME";
    match env::var_os(key) {
        Some(val) => println!("{}: {:?}", key, val),
        None => println!("{} is not defined in the environment.", key),
    }
    println!();
    for (key, value) in env::vars_os() {
        println!("{:?}: {:?}", key, value);
    }
    println!();
    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }

    println!();
    let mut dir = env::temp_dir();
    println!("Temporary directory: {}", dir.display());
}
