// #![crate_type = "bin"]
// #![crate_type = "lib"]
// #![crate_type = "dylib"]
#![crate_type = "staticlib"]
// #![crate_type = "cdylib"] //similar to dylib
// #![crate_type = "rlib"] //similar to dylib
// #![crate_type = "proc-macro"] //similar to dylib
// #![allow(unused)]
fn main() {
    #[track_caller]
    fn f() {
        println!("{}", std::panic::Location::caller());
    }
    #[track_caller]
    fn g() {
        println!("{}", std::panic::Location::caller());
        f();
    }

    fn calls_g() {
        g(); // <-- g() prints this location twice, once itself and once from f()
    }

    calls_g()
}
#[no_mangle]
pub extern "C" fn callable_from_c(x: i32) -> bool {
    x % 3 == 0
}
