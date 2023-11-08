use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

fn main() -> PyResult<()> {
    // // Initialize the Python interpreter
    // let gil = Python::acquire_gil();
    // let py = gil.python();

    // // Add the current directory to sys.path to be able to import the Python module
    // let sys = PyModule::import(py, "sys")?;
    // let path: &PyList = sys.getattr("path")?.downcast()?;
    // path.insert(0, ".")?;

    // // Import the Python module and call a function
    // let my_python_module = PyModule::import(py, "my_python_module")?;
    // let result: i32 = my_python_module.call1("add", (1, 2))?.extract()?;
    // let result: i32 = my_python_module.getattr("add")?.call1((1, 2))?.extract()?;

    // println!("Result of calling Python add function: {}", result);
    // Ok(())
    Python::with_gil(|py| {
        let sys = py.import("sys")?;
        let version: String = sys.getattr("version")?.extract()?;

        let locals = [("os", py.import("os")?)].into_py_dict(py);
        let code = "os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'";
        let user: String = py.eval(code, None, Some(&locals))?.extract()?;

        println!("Hello {}, I'm Python {}", user, version);

        let my_python_module = PyModule::import(py, "my_python_module")?;
        // let result: i32 = my_python_module.call1("add", (1, 2))?.extract()?;
        let result: i32 = my_python_module.getattr("add")?.call1((1, 2))?.extract()?;
        println!("Result of calling Python add function: {}", result);

        let result = py
            .eval("[i * 10 for i in range(5)]", None, None)
            .map_err(|e| {
                e.print_and_set_sys_last_vars(py);
            });
        let res: Vec<i64> = result.unwrap().extract().unwrap();
        assert_eq!(res, vec![0, 10, 20, 30, 40]);

        let activators = PyModule::from_code(
            py,
            r#"
import numpy

def relu(x):
    """see https://en.wikipedia.org/wiki/Rectifier_(neural_networks)"""
    return max(0.0, x)

def leaky_relu(x, slope=0.01):
    return x if x >= 0 else x * slope
"#,
            "activators.py",
            "activators",
        )?;

        let relu_result: f64 = activators.getattr("relu")?.call1((-1.0,))?.extract()?;
        assert_eq!(relu_result, 0.0);

        let kwargs = [("slope", 0.2)].into_py_dict(py);
        let lrelu_result: f64 = activators
            .getattr("leaky_relu")?
            .call((-1.0,), Some(kwargs))?
            .extract()?;
        assert_eq!(lrelu_result, -0.2);
        Ok(())
    })
}
