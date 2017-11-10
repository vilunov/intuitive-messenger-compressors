extern crate bit_vec;
extern crate byteorder;

mod huffman;

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
    wrapper(vec, len, basic_raw)
}

#[no_mangle]
pub fn huff_encode(vec: *const u8, len: usize) -> Arr {
    wrapper(vec, len, huffman::encode)
}

#[no_mangle]
pub fn huff_decode(vec: *const u8, len: usize) -> Arr {
    wrapper(vec, len, huffman::decode)
}

pub fn wrapper<F: Fn(&[u8]) -> Vec<u8>>(vec: *const u8, len: usize, func: F) -> Arr {
    let slice = unsafe { from_raw_parts(vec, len) };

    let mut vec = func(slice);
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