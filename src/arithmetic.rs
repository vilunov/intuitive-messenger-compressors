use bit_vec::BitVec;

const N: usize = 16;

fn freqs(input: &[u8]) -> [u32; 256] {
    let mut freqs: [u32; 256] = [0; 256];
    for i in input {
        freqs[*i as usize] += 1;
    }
    freqs
}

fn scale_freqs(input: &[u8], freqs: &mut [u32; 256]) {
    let total = input.len() as u32 / (1 << N) + if input.len() % ( 1 << N) != 0 { 1 } else { 0 }; //number of bytes divided by 2^N rounded up
    for i in 0..256 {
        if freqs[i] == 0 { continue; }
        freqs[i] = (freqs[i] / total).max(1);
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
    let mut freqs = freqs(input);
    scale_freqs(input, &mut freqs);
    let freqs = model(&freqs);

    let mut output = BitVec::new();

    let sum = freqs[256];
    let mut lower = 0;
    let mut upper = sum;
    println!("sum {}", sum);
    for i in input {
        let cur = upper - lower + 1;
        println!("{} {} {}", *i, freqs[*i as usize], freqs[*i as usize + 1]);
        upper = lower + cur * freqs[*i as usize + 1] / sum - 1;
        lower = lower + cur * freqs[*i as usize] / sum;
        println!("{} {} {} {} {}", cur, lower, upper, freqs[*i as usize], freqs[*i as usize - 1]);
    }
    Vec::new()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = vec![1, 3, 3, 7];
        let encoded = encode(&input[..]);
    }
}