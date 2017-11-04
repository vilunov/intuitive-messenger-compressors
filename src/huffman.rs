use bit_vec::BitVec;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Clone, Copy)]
enum Node {
    Continuation(u8, u8),
    LeftFinish(u8, u8),
    RightFinish(u8, u8),
    BothFinish(u8, u8),
}

#[derive(Eq, PartialEq)]
struct Pair(u8, u32);
impl PartialOrd<Pair> for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.1.partial_cmp(&other.1)
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.1.cmp(&other.1)
    }
}

fn slovar(freqs: [u32; 256]) -> [Node; 256] {
    //TODO
    let mut heap = BinaryHeap::<Pair>::with_capacity(256);
    for i in 0..256 {
        heap.push(Pair(i as u8, ::std::u32::MAX - freqs[i]));
    }
    while heap.len() > 1 {
        let v1 = heap.pop().unwrap();
        let v2 = heap.pop().unwrap();
    }
    [Node::Continuation(0, 0); 256]
}

pub fn decode(input: &[u8]) -> Vec<u8> {
    let mut freqs: [u32; 256] = [0; 256];
    for i in input {
        freqs[*i as usize] += 1;
    }
    let slovar: [Node; 256] = slovar(freqs);
    let bits = BitVec::from_bytes(input);

    let mut start = 0;
    let mut vec = Vec::new();
    while start < bits.len() {
        let (byte, next) = get(&bits, start, &slovar);
        vec.push(byte);
        start = next + 1;
    }
    vec
}

pub fn encode(input: &[u8]) -> Vec<u8> {
    Vec::new()
}

fn get(bits: &BitVec, start: usize, tree: &[Node; 256]) -> (u8, usize) {
    let mut current_bit = start;
    let mut current = 0;

    loop {
        use self::Node::*;
        current = match (tree[current], bits[current_bit]) {
            (Continuation(a, _), false) => a as usize,
            (Continuation(_, b), true) => b as usize,

            (LeftFinish(a, _), false) => return (a, current_bit),
            (LeftFinish(_, b), true) => b as usize,

            (RightFinish(a, _), false) => a as usize,
            (RightFinish(_, b), true) => return (b, current_bit),

            (BothFinish(a, _), false) => return (a, current_bit),
            (BothFinish(_, b), true) => return (b, current_bit),
        };
        current_bit += 1;
    }
}

mod test {
    use super::*;
    fn test() {
        let input: Vec<u8> = vec![1, 3, 3, 7];
        let encoded = encode(&input[..]);
        let decoded = decode(&encoded[..]);
        assert_eq!(decoded, input);
    }
}