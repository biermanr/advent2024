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

#[pyfunction]
fn day2_part1(data_path: &str) -> PyResult<u32> {
    Ok(days::day2::part1(Path::new(data_path)))
}

#[pyfunction]
fn day2_part2(data_path: &str) -> PyResult<u32> {
    Ok(days::day2::part2(Path::new(data_path)))
}

#[pyfunction]
fn day3_part1(data_path: &str) -> PyResult<u32> {
    Ok(days::day3::part1(Path::new(data_path)))
}

#[pyfunction]
fn day3_part2(data_path: &str) -> PyResult<u32> {
    Ok(days::day3::part2(Path::new(data_path)))
}

#[pyfunction]
fn day4_part1(data_path: &str) -> PyResult<u32> {
    Ok(days::day4::part1(Path::new(data_path)))
}

#[pyfunction]
fn day4_part2(data_path: &str) -> PyResult<u32> {
    Ok(days::day4::part2(Path::new(data_path)))
}

#[pyfunction]
fn day5_part1(data_path: &str) -> PyResult<u32> {
    Ok(days::day5::part1(Path::new(data_path)))
}

#[pyfunction]
fn day5_part2(data_path: &str) -> PyResult<u32> {
    Ok(days::day5::part2(Path::new(data_path)))
}

#[pyfunction]
fn day6_part1(data_path: &str) -> PyResult<u32> {
    Ok(days::day6::part1(Path::new(data_path)))
}

#[pyfunction]
fn day6_part2(data_path: &str) -> PyResult<u32> {
    Ok(days::day6::part2(Path::new(data_path)))
}

#[pyfunction]
fn day7_part1(data_path: &str) -> PyResult<u64> {
    Ok(days::day7::part1(Path::new(data_path)))
}

#[pyfunction]
fn day7_part2(data_path: &str) -> PyResult<u64> {
    Ok(days::day7::part2(Path::new(data_path)))
}

#[pyfunction]
fn day8_part1(data_path: &str) -> PyResult<u32> {
    Ok(days::day8::part1(Path::new(data_path)))
}

#[pyfunction]
fn day8_part2(data_path: &str) -> PyResult<u32> {
    Ok(days::day8::part2(Path::new(data_path)))
}

#[pyfunction]
fn day9_part1(data_path: &str) -> PyResult<u64> {
    Ok(days::day9::part1(Path::new(data_path)))
}

#[pyfunction]
fn day9_part2(data_path: &str) -> PyResult<u64> {
    Ok(days::day9::part2(Path::new(data_path)))
}

#[pyfunction]
fn day10_part1(data_path: &str) -> PyResult<u32> {
    Ok(days::day10::part1(Path::new(data_path)))
}

#[pyfunction]
fn day10_part2(data_path: &str) -> PyResult<u32> {
    Ok(days::day10::part2(Path::new(data_path)))
}

#[pyfunction]
fn day11_part1(data_path: &str) -> PyResult<u32> {
    Ok(days::day11::part1(Path::new(data_path)))
}

#[pyfunction]
fn day11_part2(data_path: &str) -> PyResult<u64> {
    Ok(days::day11::part2(Path::new(data_path)))
}

#[pyfunction]
fn day12_part1(data_path: &str) -> PyResult<u32> {
    Ok(days::day12::part1(Path::new(data_path)))
}

#[pyfunction]
fn day12_part2(data_path: &str) -> PyResult<u32> {
    Ok(days::day12::part2(Path::new(data_path)))
}

#[pyfunction]
fn day13_part1(data_path: &str) -> PyResult<u32> {
    Ok(days::day13::part1(Path::new(data_path)))
}

#[pymodule]
fn advent2024(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(day1_part1, m)?)?;
    m.add_function(wrap_pyfunction!(day1_part2, m)?)?;
    m.add_function(wrap_pyfunction!(day2_part1, m)?)?;
    m.add_function(wrap_pyfunction!(day2_part2, m)?)?;
    m.add_function(wrap_pyfunction!(day3_part1, m)?)?;
    m.add_function(wrap_pyfunction!(day3_part2, m)?)?;
    m.add_function(wrap_pyfunction!(day4_part1, m)?)?;
    m.add_function(wrap_pyfunction!(day4_part2, m)?)?;
    m.add_function(wrap_pyfunction!(day5_part1, m)?)?;
    m.add_function(wrap_pyfunction!(day5_part2, m)?)?;
    m.add_function(wrap_pyfunction!(day6_part1, m)?)?;
    m.add_function(wrap_pyfunction!(day6_part2, m)?)?;
    m.add_function(wrap_pyfunction!(day7_part1, m)?)?;
    m.add_function(wrap_pyfunction!(day7_part2, m)?)?;
    m.add_function(wrap_pyfunction!(day8_part1, m)?)?;
    m.add_function(wrap_pyfunction!(day8_part2, m)?)?;
    m.add_function(wrap_pyfunction!(day9_part1, m)?)?;
    m.add_function(wrap_pyfunction!(day9_part2, m)?)?;
    m.add_function(wrap_pyfunction!(day10_part1, m)?)?;
    m.add_function(wrap_pyfunction!(day10_part2, m)?)?;
    m.add_function(wrap_pyfunction!(day11_part1, m)?)?;
    m.add_function(wrap_pyfunction!(day11_part2, m)?)?;
    m.add_function(wrap_pyfunction!(day12_part1, m)?)?;
    m.add_function(wrap_pyfunction!(day12_part2, m)?)?;
    m.add_function(wrap_pyfunction!(day13_part1, m)?)?;
    Ok(())
}
