use std::slice::from_raw_parts;
use std::mem::forget;

#[repr(C)]
pub struct Arr {
    ptr: *mut u8,
    len: usize,
    cap: usize,
}

#[no_mangle]
pub fn drop(arr: Arr) {
    let _drop = unsafe { Vec::from_raw_parts(arr.ptr, arr.len, arr.cap) };
}

#[no_mangle]
pub fn basic(vec: *const u8, len: usize) -> Arr {

    let slice = unsafe { from_raw_parts(vec, len) };

    let mut vec = basic_raw(slice);
    vec.shrink_to_fit();

    let ptr = vec.as_mut_ptr();
    let len = vec.len();
    let cap = vec.capacity();
    forget(vec);

    Arr { ptr, len, cap }
}

fn basic_raw(vec: &[u8]) -> Vec<u8> {
    let mut v = vec.to_vec();
    v.pop();
    for i in 0..v.len() {
        v[i] *= 2;
    }
    v
}