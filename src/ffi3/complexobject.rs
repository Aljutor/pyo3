use ffi3::object::*;
use std::os::raw::{c_double, c_int};

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    pub static mut PyComplex_Type: PyTypeObject;
}

#[inline]
pub unsafe fn PyComplex_Check(op: *mut PyObject) -> c_int {
    PyObject_TypeCheck(op, &mut PyComplex_Type)
}

#[inline]
pub unsafe fn PyComplex_CheckExact(op: *mut PyObject) -> c_int {
    (Py_TYPE(op) == &mut PyComplex_Type) as c_int
}

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    pub fn PyComplex_FromDoubles(real: c_double, imag: c_double) -> *mut PyObject;
    pub fn PyComplex_RealAsDouble(op: *mut PyObject) -> c_double;
    pub fn PyComplex_ImagAsDouble(op: *mut PyObject) -> c_double;
}

#[cfg(not(Py_LIMITED_API))]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Py_complex {
    pub real: c_double,
    pub imag: c_double,
}

#[cfg(not(Py_LIMITED_API))]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PyComplexObject {
    _ob_base: PyObject,
    pub cval: Py_complex,
}

#[cfg(not(Py_LIMITED_API))]
#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    pub fn _Py_c_sum(left: Py_complex, right: Py_complex) -> Py_complex;
    pub fn _Py_c_diff(left: Py_complex, right: Py_complex) -> Py_complex;
    pub fn _Py_c_neg(complex: Py_complex) -> Py_complex;
    pub fn _Py_c_prod(left: Py_complex, right: Py_complex) -> Py_complex;
    pub fn _Py_c_quot(dividend: Py_complex, divisor: Py_complex) -> Py_complex;
    pub fn _Py_c_pow(num: Py_complex, exp: Py_complex) -> Py_complex;
    pub fn _Py_c_abs(arg: Py_complex) -> c_double;
    pub fn PyComplex_FromCComplex(v: Py_complex) -> *mut PyObject;
    pub fn PyComplex_AsCComplex(op: *mut PyObject) -> Py_complex;
}
