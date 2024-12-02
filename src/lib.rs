use pyo3::prelude::*;
use std::path::Path;

mod days;

#[pyfunction]
fn day1_part1(data_path: &str) -> PyResult<u32> {
    Ok(days::day1::part1(Path::new(data_path)))
}

#[pyfunction]
fn day1_part2(data_path: &str) -> PyResult<u32> {
    Ok(days::day1::part2(Path::new(data_path)))
}

#[pymodule]
fn advent2024(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(day1_part1, m)?)?;
    m.add_function(wrap_pyfunction!(day1_part2, m)?)?;
    Ok(())
}
