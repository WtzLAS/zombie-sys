use std::{
    marker::PhantomData,
    ops::{Index, IndexMut},
};

mod ffi;

pub struct AffFunction {
    inner: *mut ffi::AffFunction,
}

impl AffFunction {
    pub fn new(slope: i128, x_shift: i64) -> AffFunction {
        AffFunction {
            inner: unsafe { ffi::aff_function_new(slope, x_shift) },
        }
    }

    pub fn eval(&self, shift: i64) -> i128 {
        unsafe { ffi::aff_function_eval(self.inner, shift) }
    }

    pub fn lt_until(&self, rhs: &AffFunction) -> Option<i128> {
        unsafe {
            let res = ffi::aff_function_lt_until(self.inner, rhs.inner);
            if res.is_null() {
                None
            } else {
                Some(*res)
            }
        }
    }

    pub fn le_until(&self, rhs: &AffFunction) -> Option<i128> {
        unsafe {
            let res = ffi::aff_function_le_until(self.inner, rhs.inner);
            if res.is_null() {
                None
            } else {
                Some(*res)
            }
        }
    }
}

impl Drop for AffFunction {
    fn drop(&mut self) {
        unsafe { ffi::aff_function_delete(self.inner) }
    }
}

pub trait KineticHeap<T>: IndexMut<usize> {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn push(&mut self, t: T, aff: &AffFunction);
    fn cur_min_value(&self) -> i128;
    fn peek(&self) -> &T;
    fn peek_mut(&mut self) -> &mut T;
    fn pop(&mut self) -> T;
    fn remove(&mut self, i: usize) -> T;
    fn time(&self) -> i64;
    fn advance_to(&mut self, new_time: i64);
}

pub struct KineticHanger<T> {
    inner: *mut ffi::KineticHanger,
    _marker: PhantomData<T>,
}

unsafe impl<T: Send> Send for KineticHanger<T> { }

impl<T> KineticHanger<T> {
    pub fn new(time: i64) -> KineticHanger<T> {
        KineticHanger {
            inner: unsafe { ffi::kinetic_hanger_new(time) },
            _marker: PhantomData::default(),
        }
    }
}

impl<T> Index<usize> for KineticHanger<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe {
            Box::leak(Box::from_raw(
                ffi::kinetic_hanger_index(self.inner, index) as *mut T
            ))
        }
    }
}

impl<T> IndexMut<usize> for KineticHanger<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe {
            Box::leak(Box::from_raw(
                ffi::kinetic_hanger_index(self.inner, index) as *mut T
            ))
        }
    }
}

impl<T> KineticHeap<T> for KineticHanger<T> {
    fn len(&self) -> usize {
        unsafe { ffi::kinetic_hanger_size(self.inner) }
    }

    fn is_empty(&self) -> bool {
        unsafe { ffi::kinetic_hanger_empty(self.inner) }
    }

    fn push(&mut self, t: T, aff: &AffFunction) {
        unsafe {
            let rp = Box::into_raw(Box::new(t));
            ffi::kinetic_hanger_push(self.inner, rp as *mut libc::c_void, aff.inner);
        }
    }

    fn cur_min_value(&self) -> i128 {
        unsafe { ffi::kinetic_hanger_cur_min_value(self.inner) }
    }

    fn peek(&self) -> &T {
        unsafe { Box::leak(Box::from_raw(ffi::kinetic_hanger_peek(self.inner) as *mut T)) }
    }

    fn peek_mut(&mut self) -> &mut T {
        unsafe { Box::leak(Box::from_raw(ffi::kinetic_hanger_peek(self.inner) as *mut T)) }
    }

    fn pop(&mut self) -> T {
        unsafe { *Box::from_raw(ffi::kinetic_hanger_pop(self.inner) as *mut T) }
    }

    fn remove(&mut self, i: usize) -> T {
        unsafe { *Box::from_raw(ffi::kinetic_hanger_remove(self.inner, i) as *mut T) }
    }

    fn time(&self) -> i64 {
        unsafe { ffi::kinetic_hanger_time(self.inner) }
    }

    fn advance_to(&mut self, new_time: i64) {
        unsafe { ffi::kinetic_hanger_advance_to(self.inner, new_time) }
    }
}

impl<T> Drop for KineticHanger<T> {
    fn drop(&mut self) {
        while !self.is_empty() {
            self.pop();
        }
        unsafe {
            ffi::kinetic_hanger_delete(self.inner);
        }
    }
}

#[test]
fn kh_rtest1() {
    let mut kh: KineticHanger<i32> = KineticHanger::new(0);
    let aff = AffFunction::new(10, 0);
    assert_eq!(kh.len(), 0);
    assert_eq!(kh.is_empty(), true);
    kh.push(114514, &aff);
    kh.advance_to(10);
    print!("{}", kh.cur_min_value());
    assert_eq!(kh.cur_min_value(), 100);
    assert_eq!(kh.len(), 1);
    assert_eq!(kh.is_empty(), false);
    assert_eq!(*kh.peek(), 114514);
    assert_eq!(kh.pop(), 114514);
    assert_eq!(kh.len(), 0);
    assert_eq!(kh.is_empty(), true);
}
