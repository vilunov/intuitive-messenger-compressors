use bit_vec::BitVec;

const REPETITION_NUM: usize = 3;

pub fn encode(input: &[u8]) -> Vec<u8> {
    let bit_vec_in = BitVec::from_bytes(input);
    let mut bit_vec_out = BitVec::from_elem(bit_vec_in.len() * REPETITION_NUM, false);

    for i in 0..bit_vec_out.len() {
        bit_vec_out.set(i, bit_vec_in[i / REPETITION_NUM]);
    }

    bit_vec_out.to_bytes()
}

pub fn decode(input: &[u8]) -> Option<Vec<u8>> {
    if input.len() % REPETITION_NUM != 0 {
        return None;
    }
    let bit_vec_in = BitVec::from_bytes(input);
    let mut bit_vec_out = BitVec::from_elem(bit_vec_in.len() / 3, false);

    for i in 0..bit_vec_out.len() {
        let sum = (0..REPETITION_NUM)
            .map(|x| bit_vec_in.get(i * 3 + x).unwrap())
            .filter(|x| *x).count();
        bit_vec_out.set(i, sum > REPETITION_NUM.checked_div(2).unwrap());
    }

    Some(bit_vec_out.to_bytes())
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
    }
    */
}
