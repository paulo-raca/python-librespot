use cpython::{PyResult, PyObject, PythonObject, ObjectProtocol, Python, PyClone, ToPyObject, NoArgs};
use futures::Future;
use tokio_core::reactor::Remote;

pub fn py_wrap_future<F, T, U>(py: Python, handle: Remote, future: F, then: T) -> PyResult<PyObject>
    where F: Future + Send + 'static,
          T: FnOnce(Python, Result<F::Item, F::Error>) -> PyResult<U> + Send + 'static,
          U: ToPyObject
{
    let futures = py.import("concurrent.futures").unwrap();
    let pyfuture = futures.call(py, "Future", NoArgs, None).unwrap();
    pyfuture.call_method(py, "set_running_or_notify_cancel", NoArgs, None)?;

    let pyfuture2 = pyfuture.clone_ref(py);

    handle.spawn(move |_| {
        future.then(move |result| {
            let gil = Python::acquire_gil();
            let py = gil.python();
            let pyvalue = then(py, result);

            if pyvalue.is_ok() {
                let _ = pyfuture2.call_method(py, "set_result", (pyvalue.ok().into_py_object(py).into_object(),), None);
            } else {
                let _ = pyfuture2.call_method(py, "set_exception", (pyvalue.err().unwrap().instance(py),), None);
            }

            Ok(())
        })
    });

    Ok(pyfuture)
}
