/*

fn usqrt(n: u64) -> u64{
    if n<2 { return n;}
    let sc = usqrt(n >> 2) << 1;
    let lc = sc+1;
    if lc*lc>n { return sc;}
    else {return lc;}
}
*/

mod problems {pub mod p001;}
use problems::p001::p001;

fn main() {
   println!("p001: {}", p001(10)); 
}

