//Hamming code [7:4]
extern crate bit_vec;
extern crate num;

use self::num::abs;
use self::bit_vec::BitVec;

/*Copy bits in range of particular positions a and b from bVec
because bVec DOES NOT HAVE IT*/
pub fn copy_bits(bVec:&BitVec, a:usize, b:usize) -> BitVec{
    let mut copied = BitVec::new();
    for x in a..b{
        copied.push(bVec.get(x).unwrap())
    }
    copied
}

/*is the number equals to power of two denoted by range of number of parity bits
 from 0 to c*/
pub fn is_power_of_two(num:usize, c:u32) -> bool{
    let mut power = false;
    for a in 0..c{
        if num as u32 == 2u32.pow(a){
            power = true;
            break;
        }
    }
    power
}

pub fn set_parity(bVec:BitVec, a:u32, b:u32, c:u32) -> BitVec{
    let mut with_parity = BitVec::from_elem(7, false);
    let mut p = 0;//temporary counter for loop
    for x in 0..with_parity.len(){
        if !is_power_of_two(x+1, c){
            with_parity.set(x, bVec.get(p).unwrap());
            p+=1;
        }
    }
    for x in 0..c{
        let mut counter = 0;//counts number of bits equal to 'true'
        let mut pos = 2u32.pow(x)-1;//works as pointer to bit of the whole word
        let mut c = 1;//works as a pointer to bit in paity range
        loop{
            if (pos >= ((with_parity.len()) as u32)){
                break;
            }
            let pow = 2u32.pow(x);
            if (with_parity.get(pos as usize).unwrap() == true){
                counter+=1;
            }
            if (c<pow) {
                pos += 1;
                c+=1;
            }
                else{
                    pos+=pow+1;
                    c = 1;
                }
        }
        if (counter%2 == 0) {
            with_parity.set((2u32.pow(x)-1) as usize, true);
        }
            else {
                with_parity.set((2u32.pow(x)-1) as usize, false);
            }
    }
    with_parity
}

pub fn to_str(vec:Vec<u8>)->String{
    let mut output = String::new();
    for x in 0..vec.len(){
        if x!=vec.len()-1{
            output+=&(vec[x] as u32).to_string();
            output+=" ";
        }
        else{
            output+=&(vec[x] as u32).to_string();
        }
    }
    output
}

pub fn encode(word:String)->String{
    let mut bv = BitVec::from_bytes(&(word.into_bytes()));
    let mut vector = BitVec::new();
    vector.push(false);
    let mut start: u32 = 0;//start of substring to add parity bits
    let mut end:u32 = 4;//end of string to add parity bits
    loop {
        if end>((bv.len()) as u32) {
            break;
        }
        let mut b = copy_bits(&bv, start as usize, end as usize);
        b = set_parity(b, start, end, 3);
        for x in 0..b.len(){
            vector.push(b.get(x).unwrap())
        }
        start+=4;
        end+=4;
    }
    if (vector.len()-1)%8!=0 {
        vector.set(0, true);
    }

   to_str(vector.to_bytes())

}
