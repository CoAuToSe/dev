// // Debugging for the macro expansions
// #![feature(trace_macros)]
// trace_macros!(false);

// External dependencies
use mymacro::Duplicate;

#[derive(Duplicate)]
struct MyStruct {
    #[dupe_me]
    x: Vec<f64>,

    y: bool,

    #[dupe_me]
    z: char,
}

fn main() {
    let foo = MyStruct {
        x: vec![1.2, 2.3],
        y: true,
        z: 'e',
    };
    let bar = MyStructDuplicated { x: foo.x, z: foo.z };
    println!("{:?}", bar);
}
