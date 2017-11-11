use bit_vec::BitVec;

pub fn encode(input: &[u8]) -> Vec<u8> {
    let bit_vec = BitVec::from_bytes(input);

    let mut is_even = true;
    for bit in &bit_vec {
        if bit == true {
            is_even = !is_even;
        }
    }

    let mut vec = input.to_vec();
    vec.push((if is_even { 0 } else { 255 }));
    vec
}

pub fn decode(input: &[u8]) -> Option<Vec<u8>> {
    let bit_vec = BitVec::from_bytes(&input[0..(input.len() - 1)]);
    let mut is_even = true;
    for bit in &bit_vec {
        if bit == true {
            is_even = !is_even;
        }
    }

    let mut oneCount = 0;
    for bit in BitVec::from_bytes(&input[input.len() - 1..input.len()]) {
        if bit {
            oneCount += 1;
        }
    }
    let is_even_byte = oneCount < 5;

    if is_even_byte == is_even {
        let mut vec = input.to_vec();
        vec.pop();
        Some(vec)
    } else {
        None
    }
}

fn countOne(bv: BitVec) {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let bytes = [0, 1, 1, 0, 1, 255];
        println!("{:?}", encode(&bytes));
        assert!(false);
    }
}