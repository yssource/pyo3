// Copyright (c) 2017-present PyO3 Project and Contributors

//! Python Async/Await Interface
//! Trait and support implementation for implementing awaitable objects
//!
//! more information on python async support
//! https://docs.python.org/3/c-api/typeobj.html#async-object-structures

use ffi;
use err::PyResult;
use callback::PyObjectCallbackConverter;
use typeob::PyTypeInfo;
use class::methods::PyMethodDef;


/// Awaitable interface
#[allow(unused_variables)]
pub trait PyAsyncProtocol<'p>: PyTypeInfo + Sized + 'static {

    fn __await__(&self) -> Self::Result where Self: PyAsyncAwaitProtocol<'p> {unimplemented!()}

    fn __aiter__(&self) -> Self::Result where Self: PyAsyncAiterProtocol<'p> {unimplemented!()}

    fn __anext__(&self) -> Self::Result where Self: PyAsyncAnextProtocol<'p> {unimplemented!()}

    fn __aenter__(&self) -> Self::Result where Self: PyAsyncAenterProtocol<'p> {unimplemented!()}

    fn __aexit__(&self,
                 exc_type: Option<Self::ExcType>,
                 exc_value: Option<Self::ExcValue>,
                 traceback: Option<Self::Traceback>)
                 -> Self::Result where Self: PyAsyncAexitProtocol<'p> { unimplemented!() }
}


pub trait PyAsyncAwaitProtocol<'p>: PyAsyncProtocol<'p> {
    type Success: ::ToPyObject;
    type Result: Into<PyResult<Self::Success>>;
}

pub trait PyAsyncAiterProtocol<'p>: PyAsyncProtocol<'p> {
    type Success: ::ToPyObject;
    type Result: Into<PyResult<Self::Success>>;
}

pub trait PyAsyncAnextProtocol<'p>: PyAsyncProtocol<'p> {
    type Success: ::ToPyObject;
    type Result: Into<PyResult<Self::Success>>;
}

pub trait PyAsyncAenterProtocol<'p>: PyAsyncProtocol<'p> {
    type Success: ::ToPyObject;
    type Result: Into<PyResult<Self::Success>>;
}

pub trait PyAsyncAexitProtocol<'p>: PyAsyncProtocol<'p> {
    type ExcType: ::FromPyObject<'p>;
    type ExcValue: ::FromPyObject<'p>;
    type Traceback: ::FromPyObject<'p>;
    type Success: ::ToPyObject;
    type Result: Into<PyResult<Self::Success>>;
}


#[doc(hidden)]
pub trait PyAsyncProtocolImpl {
    fn tp_as_async() -> Option<ffi::PyAsyncMethods>;

    fn methods() -> Vec<PyMethodDef>;
}

impl<T> PyAsyncProtocolImpl for T {
    #[inline]
    default fn tp_as_async() -> Option<ffi::PyAsyncMethods> {
        None
    }

    #[inline]
    default fn methods() -> Vec<PyMethodDef> {
        Vec::new()
    }
}

impl<'p, T> PyAsyncProtocolImpl for T where T: PyAsyncProtocol<'p> {
    #[inline]
    fn tp_as_async() -> Option<ffi::PyAsyncMethods> {
        Some(ffi::PyAsyncMethods {
            am_await: Self::am_await(),
            am_aiter: Self::am_aiter(),
            am_anext: Self::am_anext(),
        })
    }

    #[inline]
    fn methods() -> Vec<PyMethodDef> {
        let mut methods = Vec::new();

        if let Some(def) = <Self as PyAsyncAenterProtocolImpl>::__aenter__() {
            methods.push(def)
        }
        if let Some(def) = <Self as PyAsyncAexitProtocolImpl>::__aexit__() {
            methods.push(def)
        }

        methods
    }
}


trait PyAsyncAwaitProtocolImpl {
    fn am_await() -> Option<ffi::unaryfunc>;
}

impl<'p, T> PyAsyncAwaitProtocolImpl for T where T: PyAsyncProtocol<'p>
{
    #[inline]
    default fn am_await() -> Option<ffi::unaryfunc> {
        None
    }
}

impl<'p, T> PyAsyncAwaitProtocolImpl for T where T: PyAsyncAwaitProtocol<'p>
{
    #[inline]
    fn am_await() -> Option<ffi::unaryfunc> {
        py_unary_func!(PyAsyncAwaitProtocol, T::__await__, PyObjectCallbackConverter)
    }
}

trait PyAsyncAiterProtocolImpl {
    fn am_aiter() -> Option<ffi::unaryfunc>;
}

impl<'p, T> PyAsyncAiterProtocolImpl for T where T: PyAsyncProtocol<'p>
{
    #[inline]
    default fn am_aiter() -> Option<ffi::unaryfunc> {
        None
    }
}

impl<'p, T> PyAsyncAiterProtocolImpl for T where T: PyAsyncAiterProtocol<'p>
{
    #[inline]
    fn am_aiter() -> Option<ffi::unaryfunc> {
        py_unary_func!(PyAsyncAiterProtocol, T::__aiter__, PyObjectCallbackConverter)
    }
}

trait PyAsyncAnextProtocolImpl {
    fn am_anext() -> Option<ffi::unaryfunc>;
}

impl<'p, T> PyAsyncAnextProtocolImpl for T where T: PyAsyncProtocol<'p>
{
    #[inline]
    default fn am_anext() -> Option<ffi::unaryfunc> {
        None
    }
}

impl<'p, T> PyAsyncAnextProtocolImpl for T where T: PyAsyncAnextProtocol<'p>
{
    #[inline]
    fn am_anext() -> Option<ffi::unaryfunc> {
        py_unary_func!(PyAsyncAnextProtocol, T::__anext__, PyObjectCallbackConverter)
    }
}

trait PyAsyncAenterProtocolImpl {
    fn __aenter__() -> Option<PyMethodDef>;
}

impl<'p, T> PyAsyncAenterProtocolImpl for T where T: PyAsyncProtocol<'p>
{
    #[inline]
    default fn __aenter__() -> Option<PyMethodDef> {
        None
    }
}

pub trait PyAsyncAexitProtocolImpl {
    fn __aexit__() -> Option<PyMethodDef>;
}

impl<'p, T> PyAsyncAexitProtocolImpl for T where T: PyAsyncProtocol<'p>
{
    #[inline]
    default fn __aexit__() -> Option<PyMethodDef> {
        None
    }
}
