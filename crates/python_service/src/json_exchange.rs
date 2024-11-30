
use pyo3::prelude::*;
use serde_json::Value;

/// Converts a Rust JSON string into a Python dictionary.
pub fn rust_json_to_python(json_str: &str) -> PyResult<PyObject> {
    Python::with_gil(|py| {
        let parsed_json: Value = serde_json::from_str(json_str).expect("Invalid JSON format");
        let json_dict = parsed_json.to_object(py);
        Ok(json_dict)
    })
}

/// Converts a Python dictionary into a Rust JSON string.
pub fn python_dict_to_rust_json(py_dict: PyObject) -> PyResult<String> {
    Python::with_gil(|py| {
        let py_dict: &pyo3::types::PyDict = py_dict.extract(py)?;
        let rust_json = serde_json::to_string(py_dict).expect("Failed to convert to JSON");
        Ok(rust_json)
    })
}
