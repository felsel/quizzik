use pyo3::prelude::*;
use pyo3::types::PyList;
use r2d2_postgres::postgres::GenericClient;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Error, Read};

use serde::Deserialize;
use toml::from_str;

use r2d2;
use r2d2_postgres::{
    postgres::{error::Error as PgError, NoTls},
    PostgresConnectionManager,
};
use std::thread;

#[derive(Debug, Deserialize)]
struct Config {
    database: Database,
}

#[derive(Debug, Deserialize)]
struct Database {
    DB_USER: String,
    DB_PASS: String,
    DB_HOST: String,
    DB_PORT: String,
    DB_NAME: String,
}

#[pyfunction]
fn connect_to_db() -> Py<PyAny> {
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
                DB_HOST: "".into(),
                DB_PORT: "".into(),
                DB_NAME: "".into(),
            },
        }
    });
    println!("{:?}", config);
    let manager = PostgresConnectionManager::new(
        format!(
            "host={} user={} password={} port={} dbname={}",
            config.database.DB_HOST,
            config.database.DB_USER,
            config.database.DB_PASS,
            config.database.DB_PORT,
            config.database.DB_NAME
        )
        .parse()
        .unwrap(),
        NoTls,
    );
    let pool = r2d2::Pool::new(manager).unwrap();
    let mut titles = Vec::<String>::new();
    let handle = thread::spawn(move || -> Result<_, PgError> {
        let mut client = pool.get().unwrap();
        let rows = client.query(
            "select title from film where film.film_id between $1 and $2",
            &[&10_i32, &50_i32],
        )?;

        for row in rows {
            let title: String = row.get("title");
            let _ = &mut titles.push(title);
        }
        Ok(titles)
    })
    .join()
    .unwrap();
    // .expect("Failed to rejoin execution thread");
    let mut hm = HashMap::new();
    for (i, e) in handle.into_iter().enumerate() {
        hm.insert(i, e);
    }
    Python::with_gil(|py: Python| hm.to_object(py))
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
