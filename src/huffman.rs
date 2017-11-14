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

    fn value(&self) -> Option<u8> {
        match self {
            &Pair::Value(a, _) => Some(a),
            _ => None,
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

fn slovar(freqs: &[u32; 256]) -> [Node; 256] {
    use std::u32::MAX;
    use self::Pair::*;

    let mut heap = BinaryHeap::<Pair>::with_capacity(256);
    for i in 0..256 {
        if freqs[i] > 0 {
            heap.push(Pair::Value(i as u8, MAX - freqs[i]));
        }
    }
    let mut arr = [Continuation(0, 0); 256];
    if heap.len() == 1 {
        let val = heap.pop().unwrap().value().unwrap();
        arr[0] = BothFinish(val, val.overflowing_add(1).0);
        return arr;
    } else if heap.len() == 0 {
        arr[0] = BothFinish(0, 1);
        return arr;
    }
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

fn slovar_encode(freqs: &[u32; 256]) -> HashMap<u8, BitVec> {
    fn helper(slovar: &[Node; 256],
              mut cur_idx: u8,
              mut cur_vec: BitVec,
              map: &mut HashMap<u8, BitVec>) {
        loop {
            match slovar[cur_idx as usize] {
                Continuation(a, b) => {
                    cur_vec.push(false);
                    helper(slovar, a, cur_vec.clone(), map);

                    let last = cur_vec.len() - 1;
                    cur_vec.set(last, true);
                    cur_idx = b;
                }
                LeftFinish(a, b) => {
                    cur_vec.push(false);
                    map.insert(a, cur_vec.clone());
                    cur_idx = b;
                    let last = cur_vec.len() - 1;
                    cur_vec.set(last, true);
                }
                RightFinish(a, b) => {
                    cur_vec.push(true);
                    map.insert(b, cur_vec.clone());
                    cur_idx = a;
                    let last = cur_vec.len() - 1;
                    cur_vec.set(last, false);
                }
                BothFinish(a, b) => {
                    cur_vec.push(false);
                    map.insert(a, cur_vec.clone());
                    let last = cur_vec.len() - 1;
                    cur_vec.set(last, true);
                    map.insert(b, cur_vec);
                    break;
                }
            }
        }
    }

    let mut map: HashMap<u8, BitVec> = HashMap::new();
    let sl = slovar(freqs);
    helper(&sl, 0, BitVec::new(), &mut map);
    map
}

pub fn decode(input: &[u8]) -> Option<Vec<u8>> {
    if input.len() < 256 * 4 + 1 {
        return None;
    }
    let mut freqs: [u32; 256] = unsafe { ::std::mem::uninitialized() };
    let mut bits = BitVec::from_bytes(&input[0..input.len() - 256 * 4 - 1]);
    {
        use std::io::Cursor;
        use byteorder::*;
        let mut cursor: Cursor<&[u8]> = Cursor::new(&input[input.len() - 256 * 4 - 1..
                                                     input.len() - 1]);
        for i in 0..256 {
            let val = cursor.read_u32::<LittleEndian>().unwrap();
            freqs[i] = val;
        }
    }
    let slovar: [Node; 256] = slovar(&freqs);
    let last = input[input.len() - 1];
    if last > 0 {
        for _ in 0..(8 - last) {
            bits.pop();
        }
    }

    let mut start = 0;
    let mut vec = Vec::new();
    while start < bits.len() {
        if let Some((byte, next)) = get(&bits, start, &slovar) {
            vec.push(byte);
            start = next + 1;
        } else {
            return None;
        }
    }
    Some(vec)
}

pub fn encode(input: &[u8]) -> Vec<u8> {
    let mut freqs: [u32; 256] = [0; 256];
    for i in input {
        freqs[*i as usize] += 1;
    }
    let slovar: HashMap<u8, BitVec> = slovar_encode(&freqs);
    let mut bits = BitVec::new();
    for i in input {
        for j in &slovar[i] {
            bits.push(j);
        }
    }
    let mut vec = bits.to_bytes();
    for i in freqs.iter() {
        use byteorder::*;

        vec.write_u32::<LittleEndian>(*i).unwrap();
    }
    vec.push((bits.len() % 8) as u8);
    vec
}

fn get(bits: &BitVec, start: usize, tree: &[Node; 256]) -> Option<(u8, usize)> {
    let mut current_bit = start;
    let mut current = 0;

    loop {
        current = match (tree[current], bits[current_bit]) {
            (Continuation(a, _), false) => a as usize,
            (Continuation(_, b), true) => b as usize,

            (LeftFinish(a, _), false) => return Some((a, current_bit)),
            (LeftFinish(_, b), true) => b as usize,

            (RightFinish(a, _), false) => a as usize,
            (RightFinish(_, b), true) => return Some((b, current_bit)),

            (BothFinish(a, _), false) => return Some((a, current_bit)),
            (BothFinish(_, b), true) => return Some((b, current_bit)),
        };
        current_bit += 1;
        if current_bit >= bits.len() {
            return None;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let input: Vec<u8> = vec![1, 3, 3, 7, 2, 2, 8, 14, 88, 13, 37, 37, 14, 53, 19, 41, 19, 45];

        let encoded = encode(&input[..]);
        let decoded = decode(&encoded[..]);
        assert_eq!(decoded, input);
    }

    #[test]
    fn test_single() {
        let input: Vec<u8> = vec![1];

        let encoded = encode(&input[..]);
        let decoded = decode(&encoded[..]);
        assert_eq!(decoded, input);
    }

    #[test]
    fn test_none() {
        let input: Vec<u8> = vec![];

        let encoded = encode(&input[..]);
        let decoded = decode(&encoded[..]);
        assert_eq!(decoded, input);
    }
}
