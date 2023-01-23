use pyo3::prelude::*;
use std::collections::HashMap;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn count_word(s: String) -> Py<PyAny> {
    let mut hm = HashMap::new();
    for sub_string in s.split(" ") {
        let count = hm.entry(sub_string).or_insert(0);
        *count += 1;
    }
    Python::with_gil(|py: Python| hm.to_object(py))
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn quizziklib(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    let word_counting_module = PyModule::new(py, "word_counting")?;
    word_counting_module.add_function(wrap_pyfunction!(count_word, m)?)?;
    m.add_submodule(word_counting_module)?;
    Ok(())
}
