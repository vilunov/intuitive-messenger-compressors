use bit_vec::BitVec;
use byteorder::*;
use std::io::Cursor;

type Probability = u32;
const PRECISION: Probability = 32; //always sizeof::<Probability>() * 8
const MAX_PROB: Probability = 1 << (PRECISION - 2);
const MASK_0: Probability = 1 << (PRECISION - 1);
const MASK_1: Probability = 1 << (PRECISION - 2);

fn freqs(input: &[u8]) -> [u32; 256] {
    let mut freqs: [u32; 256] = [0; 256];
    for i in input {
        freqs[*i as usize] += 1;
    }
    let mut m = 0;
    freqs
}

fn scale_freqs(input: &[u8], freqs: &mut [u32; 256]) {
    if input.len() < MAX_PROB as usize { return }
    let rescale = input.len() as Probability / MAX_PROB + 1;
    for i in 0..256 {
        if freqs[i] == 0 { continue; }
        freqs[i] = (freqs[i] / rescale).max(1);
    }
}

fn model(freqs: &[u32; 256]) -> [u32; 257] {
    let mut model = [0; 257];
    model[0] = 0;
    for i in 0..256 {
        model[i + 1] = model[i] + freqs[i];
    }
    model
}

pub fn encode(input: &[u8]) -> Vec<u8> {
    fn write_bits(output: &mut BitVec, underflow_bits: &mut i32, upper: &mut Probability, lower: &mut Probability) {
        loop {
            if *upper & MASK_0 == *lower & MASK_0 {
                output.push(*upper & MASK_0 != 0);
                while *underflow_bits > 0 {
                    output.push(*upper & MASK_0 == 0);
                    *underflow_bits -= 1;
                }
            } else if (*lower & MASK_1 != 0) && (*upper & MASK_0 == 0) {
                *underflow_bits += 1;
                *lower &= !(MASK_0 | MASK_1);
                *upper |= MASK_1;
            } else { break }
            *lower <<= 1;
            *upper <<= 1;
            *upper |= 1;
        }
    }

    let mut freqs = freqs(input);
    scale_freqs(input, &mut freqs);
    let freqs = model(&freqs);

    let mut output = BitVec::new();

    let sum = freqs[256] as u64;
    let mut lower: Probability = 0;
    let mut upper: Probability = !0;
    let mut underflow_bits = 0;

    for i in input.iter() {
        //apply range
        let range: u64 = upper as u64 - lower as u64 + 1;
        upper = lower + (range * freqs[*i as usize + 1] as u64 / sum) as Probability - 1;
        lower = lower + (range * freqs[*i as usize] as u64 / sum) as Probability;

        write_bits(&mut output, &mut underflow_bits, &mut upper, &mut lower);
    }

    {
        output.push(lower & MASK_1 != 0);
        for _ in 0..(underflow_bits+1) {
            output.push(lower & MASK_1 == 0);
        }
    }

    let free_bits = output.len() % 8;
    let mut bytes = output.to_bytes();
    for i in freqs.iter() {
        bytes.write_u32::<LittleEndian>(*i).unwrap();
    }
    bytes.write_u32::<LittleEndian>(input.len() as u32).unwrap();
    bytes.push(free_bits as u8);

    bytes
}

pub fn decode(input: &[u8]) -> Option<Vec<u8>> {
    fn unscaled(upper: Probability, lower: Probability, code: Probability, sum: u64) -> Probability {
        let range: u64 = upper as u64 - lower as u64 + 1;
        let mut unscaled: u64 = code as u64 - lower as u64 + 1;
        unscaled *= sum;
        unscaled -= 1;
        (unscaled / range) as Probability
    }

    if input.len() < 258 * 4 + 1 {
        return None;
    }
    let mut freqs: [u32; 257] = unsafe { ::std::mem::uninitialized() };
    let mut bits = BitVec::from_bytes(&input[0..input.len() - 258 * 4 - 1]);
    let last = input[input.len() - 1];
    if last > 0 {
        for _ in 0..(8 - last) {
            bits.pop();
        }
    }
    //read the freq table and the size of data
    let mut cursor: Cursor<&[u8]> = Cursor::new(&input[input.len() - 258 * 4 - 1..
        input.len() - 1]);
    for i in 0..257 {
        let val = cursor.read_u32::<LittleEndian>().unwrap();
        freqs[i] = val;
    }
    let len = cursor.read_u32::<LittleEndian>().unwrap();

    let sum = freqs[256] as u64;

    let mut i: usize = 0;
    let mut code: Probability = 0;
    //read first bits
    for _ in 0..PRECISION.min(bits.len() as u32) {
        code <<= 1;
        if bits[i] { code |= 1 }
        i += 1;
    }
    if bits.len() < PRECISION as usize {
        code <<= PRECISION as usize - bits.len();
    }

    let mut output: Vec<u8> = Vec::new();
    let mut lower: Probability = 0;
    let mut upper: Probability = !0;
    for _ in 0..len {
        //binary search for the val in freq table
        let val = {
            let mut first: u8 = 0;
            let mut last: u8 = 255;
            let mut middle: u8 = last / 2;
            let unscl = unscaled(upper, lower, code, sum);
            loop {
                if unscl < freqs[middle as usize] {
                    last = middle - 1;
                    middle = first + (last - first) / 2;
                    continue
                }
                if unscl >= freqs[middle as usize + 1] {
                    first = middle + 1;
                    middle = first + (last - first) / 2;
                    continue
                }
                break middle;
            }
        };
        output.push(val);

        //apply range to remove the value from the code
        {
            let range: u64 = upper as u64 - lower as u64 + 1;
            upper = lower + (range * freqs[val as usize + 1] as u64 / sum) as Probability - 1;
            lower = lower + (range * freqs[val as usize] as u64 / sum) as Probability;
        }

        //reading
        loop {
            if upper & MASK_0 == lower & MASK_0 {
            } else if (lower & MASK_1 != 0) && (upper & MASK_0 == 0) {
                lower &= !(MASK_0 | MASK_1);
                upper |= MASK_1;
                code ^= MASK_1;
            } else { break }
            lower <<= 1;
            upper <<= 1;
            upper |= 1;
            code <<= 1;
            if (i as usize) < bits.len() {
                if bits[i as usize] { code |= 1 }
                i += 1;
            }
        }
    }
    Some(output)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = vec![1, 3, 3, 7, 4, 4, 4, 4, 4, 4, 4, 4];
        let encoded = encode(&input[..]);
        let decoded = decode(&encoded[..]);
        assert_eq!(input, decoded.unwrap());
    }
}