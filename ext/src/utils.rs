use cpython::*;
use std::time::Duration;


#[allow(non_snake_case)]
pub struct Exceptions {
    pub CancelledError: PyType,
    pub InvalidStateError: PyType,
    pub TimeoutError: PyType,
}

lazy_static! {
    pub static ref EXC: Exceptions = {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let m = py.import("tokio.exceptions").unwrap();
        let cancelled = m.get(py, "CancelledError").unwrap().get_type(py);
        let invalid = m.get(py, "InvalidStateError").unwrap().get_type(py);
        let timeout = m.get(py, "TimeoutError").unwrap().get_type(py);

        Exceptions {CancelledError:cancelled,
                    InvalidStateError:invalid,
                    TimeoutError:timeout}
    };
}


//
// Check function arguments length
//
pub fn check_min_length(py: Python, args: &PyTuple, len: usize) -> PyResult<()> {
    if args.len(py) < len {
        Err(PyErr::new::<exc::TypeError, _>(
            py, format!("function takes at least {} arguments", len).to_py_object(py)))
    } else {
        Ok(())
    }
}


//
// convert PyFloat or PyInt into Duration
//
pub fn parse_seconds(py: Python, name: &str, value: PyObject) -> PyResult<Duration> {
    if let Ok(f) = PyFloat::downcast_from(py, value.clone_ref(py)) {
        let val = f.value(py);
        Ok(Duration::new(val.ceil() as u64, (val.fract() * 1_000_000.0) as u32))
    } else if let Ok(i) = PyInt::downcast_from(py, value) {
        Ok(Duration::new((i.value(py) * 1000) as u64, 0))
    } else {
        Err(PyErr::new::<exc::TypeError, _>(
            py, format!("'{}' must be int of float type", name).to_py_object(py)))
    }
}


//
// convert PyFloat or PyInt into u64 (milliseconds)
//
pub fn parse_millis(py: Python, name: &str, value: PyObject) -> PyResult<u64> {
    if let Ok(f) = PyFloat::downcast_from(py, value.clone_ref(py)) {
        Ok((f.value(py) * 1000.0) as u64)
    } else if let Ok(i) = PyInt::downcast_from(py, value) {
        Ok((i.value(py) * 1000) as u64)
    } else {
        Err(PyErr::new::<exc::TypeError, _>(
            py, format!("'{}' must be int of float type", name).to_py_object(py)))
    }
}