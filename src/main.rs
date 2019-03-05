/*

fn usqrt(n: u64) -> u64{
    if n<2 { return n;}
    let sc = usqrt(n >> 2) << 1;
    let lc = sc+1;
    if lc*lc>n { return sc;}
    else {return lc;}
}
*/
#[allow(dead_code)]

mod problems {pub mod p006;}
use problems::p006::p006;

fn main() {
    let res = p006(100);
    println!("p006: {}", res); 
}

