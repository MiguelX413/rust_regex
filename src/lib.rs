use crate::RegexFlag::ASCII;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use regex::Regex;

#[pyclass]
struct Pattern {
    pattern: Regex,
}

#[pymethods]
impl Pattern {
    fn findall(&self, string: &str) -> Vec<String> {
        self.pattern
            .find_iter(string)
            .map(|f| f.as_str().to_string())
            .collect::<Vec<String>>()
    }
}

#[pyclass]
enum RegexFlag {
    ASCII = 256,
    DEBUG = 128,
    IGNORECASE = 2,
    LOCALE = 4,
    MULTILINE = 8,
    DOTALL = 16,
    VERBOSE = 64,
    UNICODE = 32,
    TEMPLATE = 1,
}

#[pymethods]
impl RegexFlag {
    #[classattr]
    const A: RegexFlag = ASCII;
    #[classattr]
    const I: RegexFlag = RegexFlag::IGNORECASE;
    #[classattr]
    const L: RegexFlag = RegexFlag::LOCALE;
    #[classattr]
    const M: RegexFlag = RegexFlag::MULTILINE;
    #[classattr]
    const S: RegexFlag = RegexFlag::DOTALL;
    #[classattr]
    const X: RegexFlag = RegexFlag::VERBOSE;
    #[classattr]
    const U: RegexFlag = RegexFlag::UNICODE;
    #[classattr]
    const T: RegexFlag = RegexFlag::TEMPLATE;
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn compile(a: &str) -> PyResult<Pattern> {
    match Regex::new(a) {
        Ok(yeet) => Ok(Pattern { pattern: yeet }),
        Err(_) => Err(PyValueError::new_err("Lmao")),
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn rust_regex(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(compile, m)?)?;
    m.add_class::<Pattern>()?;
    m.add_class::<RegexFlag>()?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("A", RegexFlag::A)?;
    m.add("ASCII", RegexFlag::ASCII)?;
    m.add("DEBUG", RegexFlag::DEBUG)?;
    m.add("I", RegexFlag::I)?;
    m.add("IGNORECASE", RegexFlag::IGNORECASE)?;
    m.add("L", RegexFlag::L)?;
    m.add("LOCALE", RegexFlag::LOCALE)?;
    m.add("M", RegexFlag::M)?;
    m.add("MULTILINE", RegexFlag::MULTILINE)?;
    m.add("S", RegexFlag::S)?;
    m.add("DOTALL", RegexFlag::DOTALL)?;
    m.add("X", RegexFlag::X)?;
    m.add("VERBOSE", RegexFlag::VERBOSE)?;
    m.add("U", RegexFlag::U)?;
    m.add("UNICODE", RegexFlag::UNICODE)?;
    m.add("T", RegexFlag::T)?;
    m.add("TEMPLATE", RegexFlag::TEMPLATE)?;
    Ok(())
}
