//Hamming code [7:4]

mod encoder;
mod decoder;

extern crate bit_vec;
use self::bit_vec::BitVec;

fn main() {
    let a = String::from("Hello! How are you?\nkek");
    let coded = encoder::encode(a);
    /*let mut ppp = BitVec::new();
    ppp.push(false);
    ppp.push(false);
    ppp.push(false);
    ppp.push(true);
    ppp.push(true);
    ppp.push(true);
    ppp.push(false);*/
    let output = decoder::decode(coded);
    println!("{}", output);
}



