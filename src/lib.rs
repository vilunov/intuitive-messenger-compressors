/*
#![feature(test)]
extern crate test;
*/
extern crate bit_vec;
extern crate byteorder;
extern crate rand;

mod huffman;
mod parity_check;
mod repetition;
mod shannon_fano;
mod hamming;
mod arithmetic;

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
pub fn huff_encode(vec: *const u8, len: usize) -> Arr {
    wrapper_encode(vec, len, huffman::encode)
}

#[no_mangle]
pub fn huff_decode(vec: *const u8, len: usize) -> Arr {
    wrapper_decode(vec, len, huffman::decode)
}

pub fn wrapper_decode<F: Fn(&[u8]) -> Option<Vec<u8>>>(vec: *const u8, len: usize, func: F) -> Arr {
    let slice = unsafe { from_raw_parts(vec, len) };

    let vec = func(slice);
    match vec {
        Some(mut a) => {
            a.shrink_to_fit();

            let ptr = a.as_mut_ptr();
            let len = a.len();
            let cap = a.capacity();
            forget(a);

            Arr { ptr, len, cap }
        },
        None => Arr { ptr: 0 as *mut u8, len: 0, cap: 0}
    }
}

pub fn wrapper_encode<F: Fn(&[u8]) -> Vec<u8>>(vec: *const u8, len: usize, func: F) -> Arr {
    let slice = unsafe { from_raw_parts(vec, len) };

    let mut a = func(slice);
    a.shrink_to_fit();

    let ptr = a.as_mut_ptr();
    let len = a.len();
    let cap = a.capacity();
    forget(a);

    Arr { ptr, len, cap }
}