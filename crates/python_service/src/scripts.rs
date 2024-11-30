
use pyo3::prelude::*;
use pyo3::types::PyModule;

/// Executes a Python script and returns its output.
pub fn run_python_script(script: &str) -> PyResult<String> {
    Python::with_gil(|py| {
        let code = PyModule::from_code(py, script, "<embedded>", "embedded")?;
        let result: &str = code.get("result")?.extract()?;
        Ok(result.to_string())
    })
}
