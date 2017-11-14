use bit_vec::BitVec;
use std::collections::HashMap;

fn slovar_encode(freqs: &Vec<(u8, u32)>) -> HashMap<u8, BitVec> {
    fn update_map(map: &mut HashMap<u8, BitVec>, freqs: &Vec<(u8, u32)>, from: usize, to: usize, mut bit_vec: BitVec) {
        if from < to {
            let mid = step_encode(freqs, from, to);
            bit_vec.push(false);
            update_map(map, freqs, from, mid - 1, bit_vec.clone());
            bit_vec.pop();
            bit_vec.push(true);
            update_map(map, freqs, mid, to, bit_vec.clone());
        } else {
            map.insert(freqs.get(from).unwrap().0, bit_vec);
        }
    }
    let mut map: HashMap<u8, BitVec> = HashMap::new();
    update_map(&mut map, freqs, 0, freqs.len()-1, BitVec::new());
    map
}

fn slovar_decode(freqs: &Vec<(u8, u32)>) -> HashMap<BitVec, u8> {
    fn update_map(map: &mut HashMap<BitVec, u8>, freqs: &Vec<(u8, u32)>, from: usize, to: usize, mut bit_vec: BitVec) {
        if from < to {
            let mid = step_encode(freqs, from, to);
            bit_vec.push(false);
            update_map(map, freqs, from, mid - 1, bit_vec.clone());
            bit_vec.pop();
            bit_vec.push(true);
            update_map(map, freqs, mid, to, bit_vec.clone());
        } else {
            map.insert(bit_vec,freqs.get(from).unwrap().0);
        }
    }
    let mut map: HashMap<BitVec, u8> = HashMap::new();
    update_map(&mut map, freqs, 0, freqs.len()-1, BitVec::new());
    map
}

fn step_encode(freqs: &Vec<(u8, u32)>, from: usize, to: usize) -> usize {
    let mut sum_first = 0;
    let mut sum_last = 0;
    let mut i_first = from;
    let mut i_last = to;
    while i_first != i_last {
        if sum_first > sum_last {
            sum_last += freqs[i_last].1;
            i_last -= 1;
        } else {
            sum_first += freqs[i_first].1;
            i_first += 1;
        }
    }
    i_first
}

fn sort_freqs(freqs: &[u32; 256]) -> Vec<(u8, u32)> {
    let mut new_freqs: Vec<(u8, u32)> = Vec::new();
    for (i, val) in freqs.iter().enumerate() {
        if *val != 0 {
            new_freqs.push((i as u8, *val));
        }
    }
    new_freqs.sort_by(|a, b| b.1.cmp(&a.1));
    new_freqs
}

pub fn encode(input: &[u8]) -> Vec<u8> {
    let mut freqs: [u32; 256] = [0; 256];
    for i in input {
        freqs[*i as usize] += 1;
    }

    let sorted_freqs = sort_freqs(&freqs);
    let slovar: HashMap<u8, BitVec> = slovar_encode(&sorted_freqs);

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
    let sorted_freqs = sort_freqs(&freqs);
    let slovar: HashMap<BitVec, u8> = slovar_decode(&sorted_freqs);

    let last = input[input.len() - 1];
    if last > 0 {
        for _ in 0..(8 - last) {
            bits.pop();
        }
    }

    let mut start = 0;
    let mut vec = Vec::new();
    let mut byte = BitVec::new();

    while start < bits.len() {
        if !slovar.contains_key(&byte) {
            byte.push(bits.get(start).unwrap());
            start += 1;
        } else {
            vec.push(*slovar.get(&byte).unwrap());
            byte = BitVec::new();
        }
    }
    vec.push(*slovar.get(&byte).unwrap());
    Some(vec)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input: Vec<u8> = vec![1, 3, 3, 7, 2, 2, 8, 14, 88, 13, 37, 37, 14, 53, 19, 41, 19, 45];
        
        let encoded = encode(&input[..]);
        let decoded = decode(&encoded[..]).unwrap();

        assert_eq!(decoded, input);
    }
}