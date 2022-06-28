use pyo3::create_exception;
use pyo3::exceptions::{PyException, PyTypeError};
use pyo3::prelude::*;
use regex::Regex;

create_exception!(rust_regex, error, PyException);

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

impl Clone for Pattern {
    fn clone(&self) -> Self {
        Pattern {
            pattern: self.pattern.clone(),
        }
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
    const A: RegexFlag = RegexFlag::ASCII;
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

#[pyfunction]
fn compile(pattern: &PyAny) -> PyResult<Pattern> {
    match pattern.extract::<&str>() {
        Ok(pattern) => match Regex::new(pattern) {
            Ok(pattern) => Ok(Pattern { pattern }),
            Err(err) => Err(error::new_err(err.to_string())),
        },
        Err(_) => match pattern.extract::<Pattern>() {
            Ok(pattern) => Ok(pattern),
            Err(_) => Err(PyTypeError::new_err(
                "first argument must be string or compiled pattern",
            )),
        },
    }
}

#[pyfunction]
fn findall(pattern: &PyAny, string: &str) -> PyResult<Vec<String>> {
    Ok(compile(pattern)?.findall(string))
}

/// A Python module implemented in Rust.
#[pymodule]
fn rust_regex(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("error", _py.get_type::<error>())?;
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
