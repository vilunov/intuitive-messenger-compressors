use bit_vec::BitVec;
use std::collections::HashSet;

/*Copy bits in range of particular positions a and b from bVec
because bVec DOES NOT HAVE IT*/
pub fn copy_bits(bVec: &BitVec, a: usize, b: usize) -> BitVec {
    let mut copied = BitVec::new();
    for x in a..b {
        copied.push(bVec.get(x).unwrap())
    }
    copied
}

/*is the number equals to power of two denoted by range of number of parity bits
 from 0 to c*/
pub fn is_power_of_two(num: usize, c: u32) -> bool {
    let mut power = false;
    for a in 0..c {
        if num as u32 == 2u32.pow(a) {
            power = true;
            break;
        }
    }
    power
}

pub fn set_parity(bVec: BitVec, a: u32, b: u32, c: u32) -> BitVec {
    let mut with_parity = BitVec::from_elem(7, false);
    let mut p = 0; //temporary counter for loop
    for x in 0..with_parity.len() {
        if !is_power_of_two(x + 1, c) {
            with_parity.set(x, bVec.get(p).unwrap());
            p += 1;
        }
    }
    for x in 0..c {
        let mut counter = 0; //counts number of bits equal to 'true'
        let mut pos = 2u32.pow(x) - 1; //works as pointer to bit of the whole word
        let mut c = 1; //works as a pointer to bit in paity range
        loop {
            if pos >= with_parity.len() as u32 {
                break;
            }
            let pow = 2u32.pow(x);
            if with_parity.get(pos as usize).unwrap() == true {
                counter += 1;
            }
            if c < pow {
                pos += 1;
                c += 1;
            } else {
                pos += pow + 1;
                c = 1;
            }
        }
        if counter % 2 == 0 {
            with_parity.set((2u32.pow(x) - 1) as usize, true);
        } else {
            with_parity.set((2u32.pow(x) - 1) as usize, false);
        }
    }
    with_parity
}



pub fn encode(input: &[u8])->Vec<u8>{
    let mut bv = BitVec::from_bytes(input);
    let mut vector = BitVec::new();
    vector.push(false);
    let mut start: u32 = 0; //start of substring to add parity bits
    let mut end: u32 = 4; //end of string to add parity bits
    loop {
        if end > ((bv.len()) as u32) {
            break;
        }
        let mut b = copy_bits(&bv, start as usize, end as usize);
        b = set_parity(b, start, end, 3);
        for x in 0..b.len() {
            vector.push(b.get(x).unwrap())
        }
        start += 4;
        end += 4;
    }
    if (vector.len() - 1) % 8 != 0 {
        vector.set(0, true);
    }

   vector.to_bytes()

}

pub fn append(vec1: BitVec, vec2: BitVec) -> BitVec {
    let mut vec3 = vec1;
    for x in 0..vec2.len() {
        vec3.push(vec2.get(x).unwrap());
    }
    vec3
}


pub fn decode(coded:&[u8])->Vec<u8>{
    let mut word = BitVec::from_bytes(coded);
    let mut encoded_whole = BitVec::new();
    word = checking_for_eight(word);
    let mut start = 0;
    let mut end = 7;
    loop {
        if end > word.len() {
            break;
        }
        let word1 = copy_bits(&word, start, end);
        let encoded = parity_check(word1);
        encoded_whole = append(encoded_whole, encoded);
        start += 7;
        end += 7;
    }
    let vec = &encoded_whole.to_bytes();
    
    vec
}

pub fn checking_for_eight(code: BitVec) -> BitVec {
    let mut trunc = BitVec::new();
    if code.get(0).unwrap() == false {
        trunc = copy_bits(&code, 1, code.len() - 7);
    } else {
        trunc = copy_bits(&code, 1, code.len() - 1 - (7 - (code.len() - 1) % 4));
    }
    trunc
}

pub fn indexes_to_check(a: u32, b: u32) -> HashSet<u32> {
    let mut set: HashSet<u32> = HashSet::new();
    let mut k = 2u32.pow(a) - 1;
    let mut pp = 1;
    loop {
        if k > b {
            break;
        }
        if k != 2u32.pow(a) - 1 {
            set.insert(k);
        }
        if pp == 2u32.pow(a) {
            k += 2u32.pow(a) + 1;
            pp = 1;
        } else {
            k += 1;
            pp += 1;
        }
    }
    set
}

pub fn count_mistakes(a: &BitVec, ln: u32) -> u32 {
    let mut aa: u32 = 0;
    for x in 0..ln {
        if a.get(x as usize).unwrap() == false {
            aa += 1;
        }
    }
    aa
}

pub fn flipped_bit(index: Vec<HashSet<u32>>, parity: BitVec, code: BitVec) -> BitVec {
    let mut clear_vec = BitVec::new();
    let mut decoded = code;
    let mut ind1: usize = 4;
    let mut ind2: usize = 4;
    let mut ind3: usize = 4;
    let ln = parity.len();
    if !(parity.all()) {
        if count_mistakes(&parity, ln as u32) == 2 {
            for x in 0..ln {
                if (parity).get(x).unwrap() == false {
                    if ind1 == 4 {
                        ind1 = x;
                    } else {
                        ind2 = x;
                    }
                } else {
                    ind3 = x;
                }
            }
            let pos_mistake: HashSet<u32> = index[ind1]
                .intersection(&index[ind2])
                .cloned()
                .collect();
            let mistake: Vec<u32> = pos_mistake.difference(&index[ind3]).cloned().collect();

            let bit_to_flip: usize = mistake[0] as usize;
            let val = decoded.get(bit_to_flip).unwrap();
            decoded.set(bit_to_flip, !val);
        } else {
            let val = decoded.get(6).unwrap();
            decoded.set(6, !val);
        }

    }

    let mut final_decoded = BitVec::new();
    for x in 0..decoded.len() {
        if ((x + 1) as f64).log2().round() != ((x + 1) as f64).log2() {
            final_decoded.push(decoded.get(x).unwrap());
        }
    }
    final_decoded
}


pub fn parity_check(code: BitVec) -> BitVec {
    let mut parity = BitVec::from_elem(3, false);
    let mut indexes: Vec<HashSet<u32>> = Vec::new();

    for x in 0..3 {
        let mut av_indexes = indexes_to_check(x, 7);
        let mut kk = false;
        let mut counter = 0;
        for y in 0..code.len() {
            if av_indexes.contains(&(y as u32)) {
                if code.get(y).unwrap() == true {
                    counter += 1;
                }
            }
        }
        indexes.push(av_indexes);
        if counter % 2 == 0 {
            kk = true;
        }
        if code.get(2usize.pow(x) - 1).unwrap() != kk {
            parity.set(x as usize, false);
        } else {
            parity.set(x as usize, true);
        }
    }

    let mut encoded = flipped_bit(indexes, parity, code);
    encoded
}
