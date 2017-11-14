use bit_vec::BitVec;

const N: usize = 4;

pub fn encode(input: &[u8]) -> Vec<u8> {
    let mut vec = Vec::new();

    let mut pos = 0;
    while pos < input.len() {
        let bit_vec = BitVec::from_bytes(&input[pos..(pos + N).min(input.len())]);

        let is_even = bit_vec.iter().fold(!bit_vec[0], |a, b| a != b);

        vec.append(&mut bit_vec.to_bytes());
        vec.push((if is_even { 0 } else { 255 }));

        pos += N;
    }

    vec
}

pub fn decode(input: &[u8]) -> Option<Vec<u8>> {
    let mut vec = Vec::new();

    let mut pos = 0;
    while pos < input.len() {
        let check_byte_pos = (pos + N).min(input.len() - 1);

        let bit_vec = BitVec::from_bytes(&input[pos..check_byte_pos]);
        let is_even = bit_vec.iter().fold(!bit_vec[0], |a, b| a != b);

        let mut one_count = 0;
        for bit in BitVec::from_bytes(&input[check_byte_pos..check_byte_pos + 1]) {
            one_count += bit as u32;
        }
        let is_even_byte = one_count < 5;
        if is_even_byte != is_even {
            return None;
        }

        vec.append(&mut bit_vec.to_bytes());

        pos += N + 1;
    }

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

    /*
    use ::test::{Bencher, black_box};
    use ::rand::{thread_rng, Rng};

    #[bench]
    fn bench1(b: &mut Bencher) {
        const CAPACITY: usize = 1 * 1024;
        let mut rng = thread_rng();
        let mut vec = Vec::with_capacity(CAPACITY);
        for _ in 0..CAPACITY {
            vec.push(rng.gen::<u8>());
        }
        b.iter(|| {
            black_box(encode(&vec));
        });
    }

    #[bench]
    fn bench2(b: &mut Bencher) {
        const CAPACITY: usize = 1 * 1024;
        let mut rng = thread_rng();
        let mut vec = Vec::with_capacity(CAPACITY);
        for _ in 0..CAPACITY {
            vec.push(rng.gen::<u8>());
        }
        let vec2 = encode(&vec);
        b.iter(|| {
            black_box(decode(&vec2));
        });
    }*/
}
