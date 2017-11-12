use bit_vec::BitVec;

const REPETITION_NUM: usize = 3;

pub fn encode(input: &[u8]) -> Vec<u8> {
    let bit_vec_in = BitVec::from_bytes(input);
    let mut bit_vec_out = BitVec::new();

    for bit in &bit_vec_in {
        for _ in 0..3 {
            bit_vec_out.push(bit);
        }
    }

    bit_vec_out.to_bytes()
}

pub fn decode(input: &[u8]) -> Option<Vec<u8>> {
    fn read_seq(in_vec: &BitVec<u32>, i: usize) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        for x in 0..REPETITION_NUM {
            out.push(in_vec.get(i + x).unwrap() as u8);
        }
        out
    }

    fn sum(in_vec: Vec<u8>) -> u8 {
        let mut out: u8 = 0;
        for x in &in_vec {
            out += *x;
        }
        out
    }

    if input.len() % REPETITION_NUM != 0 {
        return None;
    }

    let bit_vec_in = BitVec::from_bytes(input);

    let mut bit_vec_out = BitVec::new();

    let mut i = 0;

    for _ in 0..bit_vec_in.len()/3 {
        let sum = sum(read_seq(&bit_vec_in, i));

        if sum > 1 {
            bit_vec_out.push(true);
        } else {
            bit_vec_out.push(false);
        }

        i += 3;
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
}