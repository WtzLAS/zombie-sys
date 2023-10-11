use std::ptr::null_mut;

#[repr(C)]
pub struct AffFunction {
    _private: [u8; 0],
}

#[repr(C)]
pub struct KineticHanger {
    _private: [u8; 0],
}

extern "C" {
    pub fn aff_function_new(slope: i128, x_shift: i64) -> *mut AffFunction;
    pub fn aff_function_eval(aff_function: *const AffFunction, shift: i64) -> i128;
    pub fn aff_function_lt_until(
        aff_function: *const AffFunction,
        rhs: *const AffFunction,
    ) -> *mut i128;
    pub fn aff_function_le_until(
        aff_function: *const AffFunction,
        rhs: *const AffFunction,
    ) -> *mut i128;
    pub fn aff_function_delete(aff_function: *mut AffFunction);

    pub fn kinetic_hanger_new(time: i64) -> *mut KineticHanger;
    pub fn kinetic_hanger_size(hanger: *const KineticHanger) -> libc::size_t;
    pub fn kinetic_hanger_empty(hanger: *const KineticHanger) -> bool;
    pub fn kinetic_hanger_push(
        hanger: *mut KineticHanger,
        t: *mut libc::c_void,
        aff: *const AffFunction,
    );
    pub fn kinetic_hanger_peek(hanger: *mut KineticHanger) -> *mut libc::c_void;
    pub fn kinetic_hanger_index(hanger: *mut KineticHanger, i: libc::size_t) -> *mut libc::c_void;
    pub fn kinetic_hanger_pop(hanger: *mut KineticHanger) -> *mut libc::c_void;
    pub fn kinetic_hanger_remove(hanger: *mut KineticHanger, i: libc::size_t) -> *mut libc::c_void;
    pub fn kinetic_hanger_time(hanger: *const KineticHanger) -> i64;
    pub fn kinetic_hanger_advance_to(hanger: *mut KineticHanger, new_time: i64);
    pub fn kinetic_hanger_delete(hanger: *mut KineticHanger);
}

#[test]
fn aff_test1() {
    unsafe {
        let aff = aff_function_new(10, 0);
        let res = aff_function_eval(aff, 1);
        assert_eq!(res, 10);
    }
}

#[test]
fn aff_test2() {
    unsafe {
        let aff = aff_function_new(10, 0);
        let aff2 = aff_function_new(10, 1);
        let res = aff_function_lt_until(aff, aff2);
        assert_eq!(res, null_mut());
    }
}

#[test]
fn aff_test3() {
    unsafe {
        let aff = aff_function_new(10, 0);
        let aff2 = aff_function_new(5, 1);
        let res = aff_function_lt_until(aff, aff2);
        assert_ne!(res, null_mut());
        assert_eq!(*res, 1);
    }
}

#[test]
fn kh_test1() {
    unsafe {
        let kh = kinetic_hanger_new();
        let sz = kinetic_hanger_size(kh);
        let e = kinetic_hanger_empty(kh);
        assert_eq!(sz, 0);
        assert_eq!(e, true);
    }
}

#[test]
fn kh_test2() {
    unsafe {
        let kh = kinetic_hanger_new();
        let aff = aff_function_new(10, 0);
        let t = Box::into_raw(Box::new(114514));
        kinetic_hanger_push(kh, t as *mut libc::c_void, aff);
        let sz = kinetic_hanger_size(kh);
        let e = kinetic_hanger_empty(kh);
        assert_eq!(sz, 1);
        assert_eq!(e, false);
        let ot1 = kinetic_hanger_pop(kh) as *mut i32;
        assert_ne!(ot1, null_mut());
        let ot2 = Box::from_raw(ot1);
        assert_eq!(*ot2, 114514);
    }
}
