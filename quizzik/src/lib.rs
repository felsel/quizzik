use pyo3::prelude::*;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Error, Read};

use serde::Deserialize;
use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::Pool;
use toml::from_str;

#[derive(Debug, Deserialize)]
struct Config {
    database: Database,
}

#[derive(Debug, Deserialize)]
struct Database {
    DB_USER: String,
    DB_PASS: String,
    DB_URL: String,
}

#[pyfunction]
fn connect_to_db() -> PyResult<String> {
    // let mut buf: Vec<u8> = vec![];
    // let mut file = File::open("Config.toml")?;
    // file.read_to_end(&mut buf).expect("failed to read file");
    // let config = String::from_utf8(buf).expect("failed to convert config bytes to string");

    let config =
        fs::read_to_string("Config.toml").expect("failed to convert config bytes to string");
    let config: Config = toml::from_str(&config).unwrap_or_else(|e| {
        println!(
            "Failed to create Config struct out of config file's content. Error: {}",
            e.message()
        );
        Config {
            database: Database {
                DB_USER: "".into(),
                DB_PASS: "".into(),
                DB_URL: "".into(),
            },
        }
    });
    println!("{:?}", config);
    Ok(config.database.DB_USER)
    // let pool = PgPoolOptions::max_connections(10).connect(self, format!(BD_URL, DB_USER, DB_PASS));
}

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
    m.add_function(wrap_pyfunction!(connect_to_db, m)?)?;
    Ok(())
}
