extern crate bit_vec;
extern crate num;

use self::num::abs;
use self::bit_vec::BitVec;
use std::collections::HashSet;


/*Copy bits in range of particular positions a and b from bVec
because bVec DOES NOT HAVE IT*/
pub fn copy_bits(bVec:&BitVec, a:usize, b:usize) -> BitVec{
    let mut ba = BitVec::new();
    for x in a..b{
        ba.push(bVec.get(x).unwrap())
    }
    ba
}


pub fn append(vec1:BitVec, vec2:BitVec)->BitVec{
    let mut vec3 = vec1;
    for x in 0..vec2.len(){
        vec3.push(vec2.get(x).unwrap());
    }
    vec3
}

pub fn parse_to_u8(line:Vec<&str>)->Vec<u8>{
    let mut output:Vec<u8>= Vec::new();
    for x in 0..line.len(){
        output.push(line[x].parse().unwrap());
    }
    output
}

pub fn decode(coded:String)->String{
    let v: Vec<&str> = coded.split(' ').collect();
    //println!("{}", v.len());
    let parsed = parse_to_u8(v);
    let mut word = BitVec::from_bytes(&parsed);
    let mut encoded_whole = BitVec::new();
    word = checking_for_eight(word);
    let mut start = 0;
    let mut end = 7;
    loop{
        if (end>word.len()){
            break;
        }
        let word1 = copy_bits(&word, start, end);
        let encoded = parity_check(word1);
        encoded_whole = append(encoded_whole, encoded);
        start+=7;
        end+=7;
    }
    let vec = encoded_whole.to_bytes();
    let decoded= String::from_utf8_lossy(&vec).into_owned();
    decoded
}

pub fn checking_for_eight(code: BitVec) -> BitVec{
    let mut trunc = BitVec::new();
    if code.get(0).unwrap() == false {
        trunc = copy_bits(&code, 1, code.len()-7);
    }
    else {
        trunc = copy_bits(&code, 1, code.len()-1 -(7-(code.len()-1)%4));
    }
    trunc
}

pub fn indexes_to_check(a:u32, b:u32)->HashSet<u32>{
    let mut set:HashSet<u32> = HashSet::new();
    let mut k = 2u32.pow(a)-1;
    let mut pp = 1;
    loop {
        if k>b {
            break;
        }
        if k!= 2u32.pow(a)-1{
            set.insert(k);
        }
        if (pp==2u32.pow(a)){
            k+=2u32.pow(a)+1;
            pp=1;
        }
        else {
            k+=1;
            pp+=1;
        }
    }
    set
}

pub fn count_mistakes(a:&BitVec, ln:u32)->u32{
    let mut aa:u32 = 0;
    for x in 0..ln{
        if a.get(x as usize).unwrap() == false{
            aa+=1;
        }
    }
    aa
}

pub fn flipped_bit(index:Vec<HashSet<u32>>, parity:BitVec, code:BitVec)->BitVec{
    let mut clear_vec = BitVec::new();
    let mut decoded = code;
    let mut ind1:usize =4;
    let mut ind2:usize = 4;
    let mut ind3:usize = 4;
    let ln = parity.len();
    if ! (parity.all()){
        if count_mistakes(&parity, ln as u32)==2{
           for x in 0..ln{
               if (parity).get(x).unwrap()==false {
                   if (ind1==4){
                       ind1 = x;
                   }
                   else {
                       ind2 = x;
                   }
               }
               else {
                   ind3 = x;
               }
           }
            let pos_mistake:HashSet<u32>= index[ind1].intersection(&index[ind2]).cloned().collect();
            let mistake:Vec<u32> = pos_mistake.difference(&index[ind3]).cloned().collect();

            let bit_to_flip:usize = mistake[0] as usize;
            if (decoded.get(bit_to_flip ).unwrap()==true){
                decoded.set(bit_to_flip , false);
            }
            else { decoded.set(bit_to_flip , true); }
        }
        else {
            if (decoded.get(6 ).unwrap()==true){
                decoded.set(6 , false);
            }
                else { decoded.set(6, true); }
        }

    }

    let mut final_decoded = BitVec::new();
    for x in 0..decoded.len(){
        if ((x+1) as f64).log2().round()!=((x+1) as f64).log2(){
            final_decoded.push(decoded.get(x).unwrap());
        }
    }
    final_decoded
}


pub fn parity_check(code:BitVec)->BitVec{
    let mut parity = BitVec::from_elem(3, false);
    let mut indexes : Vec<HashSet<u32>> = Vec::new();

    for x in 0..3 {
        let mut av_indexes = indexes_to_check(x, 7);
        let mut kk = false;
        let mut counter = 0;
        for y in 0..code.len(){
            if av_indexes.contains(&(y as u32)) {
                if code.get(y).unwrap() == true{
                    counter+=1;
                }
            }
        }
        indexes.push(av_indexes);
        if counter%2 == 0 {
            kk = true;
        }
        if (code.get((2u32.pow(x)-1) as usize).unwrap()!=kk){
            parity.set(x as usize, false);
        }
        else {
            parity.set(x as usize, true);
        }
    }

    let mut encoded = flipped_bit(indexes, parity, code);
    encoded
}