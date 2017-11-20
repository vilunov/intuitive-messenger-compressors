use bit_vec::BitVec;

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
    println!("sum {}", sum);

    for i in input {
        println!("up {}; low {}", upper, lower);
        let range: u64 = upper as u64 - lower as u64 + 1;
        println!("range {}", range);

        println!("{} {} {}", *i, freqs[*i as usize], freqs[*i as usize + 1]);
        upper = lower + (range * freqs[*i as usize + 1] as u64 / sum) as Probability - 1;
        lower = lower + (range * freqs[*i as usize] as u64 / sum) as Probability;
        println!("{} {} {} {} {}", range, lower, upper, freqs[*i as usize], freqs[*i as usize - 1]);
        write_bits(&mut output, &mut underflow_bits, &mut upper, &mut lower);
    }
    output.push(lower & MASK_1 != 0);
    for _ in 0..underflow_bits {
        output.push(lower & MASK_1 == 0);
    }

    let freebits = output.len() % 8;
    let mut bytes = output.to_bytes();
    for i in freqs.iter() {
        use byteorder::*;
        bytes.write_u32::<LittleEndian>(*i).unwrap();
    }
    bytes.push(freebits as u8);
    bytes
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = vec![1, 3, 3, 7];
        let encoded = encode(&input[..]);
        println!("encoded {:?}", encoded);
    }
}