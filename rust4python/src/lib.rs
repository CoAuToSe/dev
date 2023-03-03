// use pyo3::prelude::*;

// #[pyfunction]
// fn greet(name: &str) -> PyResult<String> {
//     Ok(format!("Hello, {}!", name))
// }

// /// Formats the sum of two numbers as string.
// #[pyfunction]
// fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
//     Ok((a + b).to_string())
// }

// /// A Python module implemented in Rust. The name of this function must match
// /// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
// /// import the module.
// #[pymodule]
// fn string_sum(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
//     m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
//     Ok(())
// }

// /// need the same name as library
// #[pymodule]
// fn rust4python(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
//     m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
//     Ok(())
// }

#[no_mangle]
pub extern "C" fn lib_test() {
    println!("Hello from the library!");
}

#[no_mangle]
pub extern "C" fn wtf(par: usize) {
    println!("{}", par);
}

fn do_it(sili: &str) -> String {
    return format!("Hello, {}!", sili);
}

#[inline]
#[no_mangle]
pub fn greet(name: &str) -> Result<String, ()> {
    // return Ok(format!("Hello, {}!", name));
    // return Ok(do_it(name));
    return Ok("Hello,!".to_string());
}

/// Formats the sum of two numbers as string.
#[no_mangle]
pub fn sum_as_string(a: usize, b: usize) -> Result<String, ()> {
    let temp = a + b;
    let za = temp.to_string();
    return Ok(za);
}

#[no_mangle]
pub fn sum_as_string_fail(a: usize, b: usize) -> Result<String, ()> {
    return Ok((a + b).to_string());
}

// /// A Python module implemented in Rust. The name of this function must match
// /// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
// /// import the module.
// #[no_mangle]
// fn string_sum(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
//     // m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;

//     {
//         use sum_as_string as wrapped_pyfunction;
//         pyo3::impl_::pyfunction::wrap_pyfunction(&wrapped_pyfunction::DEF, m)
//     };
//     Ok(())
// }

// /// need the same name as library
// #[no_mangle]
// fn rust4python(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
//     // m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
//     {
//         use sum_as_string as wrapped_pyfunction;
//         pyo3::impl_::pyfunction::wrap_pyfunction(&wrapped_pyfunction::DEF, m)
//     };
//     Ok(())
// }
