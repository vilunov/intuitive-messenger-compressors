const N: usize = 16;

pub fn encode(input: &[u8]) -> Vec<u8> {
    let mut freqs: [u32; 256] = [0; 256];
    for i in input {
        freqs[*i as usize] += 1;
    }
    let total = (input.len() as u32 * 2 - 1) >> N;
    let mut sum = 0;
    for i in 0..256 {
        if freqs[i] > 0 {
            freqs[i] = (freqs[i] / total).max(1);
            sum += freqs[i];
        }
    }

    let mut lower = 0;
    let mut upper = input.len();
    for i in input {
        let cur = upper - lower + 1;
        upper = lower = ((cur * ))
    }
}