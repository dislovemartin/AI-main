use pyo3::prelude::*;
use pyo3::types::PyString;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PyServiceError {
    #[error("Initialization error: {0}")]
    InitError(#[from] pyo3::PyErr),
    #[error("Execution error: {0}")]
    ExecError(#[from] pyo3::PyErr),
}

#[pyfunction]
fn example_function() -> PyResult<String> {
    Python::with_gil(|py| {
        let greeting: &PyString = py.eval("'Hello from Python'", None, None)?;
        Ok(greeting.to_str()?.to_string())
    })
}

#[pymodule]
fn python_service(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(example_function, m)?)?;
    Ok(())
}
