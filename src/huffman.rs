use bit_vec::BitVec;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum Node {
    Continuation(u8, u8),
    LeftFinish(u8, u8),
    RightFinish(u8, u8),
    BothFinish(u8, u8),
}

use self::Node::*;

#[derive(Eq, PartialEq)]
enum Pair {
    Pointer(u8, u32),
    Value(u8, u32),
}

impl Pair {
    fn prob(&self) -> u32 {
        match self {
            &Pair::Pointer(_, a) => a,
            &Pair::Value(_, a) => a,
        }
    }
}

impl PartialOrd<Pair> for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.prob().partial_cmp(&other.prob())
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.prob().cmp(&other.prob())
    }
}

fn slovar(freqs: [u32; 256]) -> [Node; 256] {
    use std::u32::MAX;
    use self::Pair::*;

    let mut heap = BinaryHeap::<Pair>::with_capacity(256);
    for i in 0..256 {
        if freqs[i] > 0 {
            heap.push(Pair::Value(i as u8, MAX - freqs[i]));
        }
    }
    let mut arr = [Node::Continuation(0, 0); 256];
    let mut cur: u8 = 255;
    while heap.len() > 1 {
        cur -= 1;
        let v1 = heap.pop().unwrap();
        let v2 = heap.pop().unwrap();
        let prob = MAX - ((MAX - v1.prob()) + (MAX - v2.prob()));
        arr[cur as usize] = match (v1, v2) {
            (Pointer(a, _), Pointer(b, _)) => Continuation(a, b),
            (Value(a, _), Pointer(b, _)) => LeftFinish(a, b),
            (Pointer(a, _), Value(b, _)) => RightFinish(a, b),
            (Value(a, _), Value(b, _)) => BothFinish(a, b),
        };
        heap.push(Pair::Pointer(cur, prob));
    }
    arr[0] = arr[cur as usize];
    arr
}

fn slovar_encode(freqs: [u32; 256]) -> HashMap<u8, BitVec> {
    use std::u32::MAX;
    use self::Pair::*;

    let mut map: HashMap<u8, BitVec> = HashMap::new();
    let mut heap = BinaryHeap::<Pair>::with_capacity(256);
    for i in 0..256 {
        if freqs[i] > 0 {
            heap.push(Pair::Value(i as u8, MAX - freqs[i]));
        }
        map.insert(i as u8, BitVec::new());
    }
    map
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
    let mut freqs: [u32; 256] = [0; 256];
    for i in input {
        freqs[*i as usize] += 1;
    }
    let slovar: HashMap<u8, BitVec> = slovar_encode(freqs);
    for (a, b) in slovar {
        println!("{} {:?}", a, b);
    }
    Vec::new()
}

fn get(bits: &BitVec, start: usize, tree: &[Node; 256]) -> (u8, usize) {
    let mut current_bit = start;
    let mut current = 0;

    loop {
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let mut input: Vec<u8> = vec![1, 3, 3, 7];

        let encoded = encode(&input[..]);
        let decoded = decode(&encoded[..]);
        assert_eq!(decoded, input);
    }
}