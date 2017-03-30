extern crate libc;
extern crate net2;
extern crate bytes;
extern crate futures;
extern crate tokio_io;
extern crate tokio_core;
extern crate tokio_signal;
extern crate boxfnonce;
#[macro_use] extern crate log;
#[macro_use] extern crate cpython;
#[macro_use] extern crate lazy_static;

use cpython::*;

mod addrinfo;
mod handle;
mod utils;
mod future;
mod remote;
mod event_loop;
mod transport;
mod server;
mod unsafepy;

pub use future::TokioFuture;
pub use handle::{TokioHandle, TokioTimerHandle};
pub use event_loop::{TokioEventLoop, new_event_loop};
pub use remote::{RemoteTokioEventLoop, spawn_event_loop};


py_module_initializer!(_ext, init_ext, PyInit__ext, |py, m| {
    m.add(py, "__doc__", "Asyncio event loop based on tokio")?;

    init_tokio_module(py, m)?;
    Ok(())
});


pub fn init_tokio_module(py: cpython::Python, m: &cpython::PyModule) -> cpython::PyResult<()> {
    m.add_class::<event_loop::TokioEventLoop>(py)?;
    m.add_class::<RemoteTokioEventLoop>(py)?;
    m.add_class::<future::TokioFuture>(py)?;
    m.add_class::<handle::TokioHandle>(py)?;
    m.add_class::<handle::TokioTimerHandle>(py)?;
    m.add_class::<server::TokioServer>(py)?;

    m.add(py, "spawn_event_loop", py_fn!(py, spawn_event_loop(name: &PyString)))?;
    m.add(py, "new_event_loop", py_fn!(py, new_event_loop()))?;
    Ok(())
}
