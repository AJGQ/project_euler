/*
fn seq_sum(n: i32) -> i32{
    return n*(n+1)/2;
}

fn p001(mut n: i32) -> i32{
    // n = 1000
    n-=1;
    return 3*seq_sum(n/3)
        +  5*seq_sum(n/5)
        -  15*seq_sum(n/15);
}

fn p002(n: i64) -> i64{
    // n = 4000000
    let mut f: [i64; 2] = [1,2];
    let mut t: i64;
    let mut r: i64 = 0;

    while f[1]<=n {
       r += f[1];

       t  = f[1];
       f[1]+= f[0];
       f[0] = t;

       t  = f[1];
       f[1]+= f[0];
       f[0] = t;

       t  = f[1];
       f[1]+= f[0];
       f[0] = t;
    }

    return r;
}

fn usqrt(n: u64) -> u64{
    if n<2 { return n;}
    let sc = usqrt(n >> 2) << 1;
    let lc = sc+1;
    if lc*lc>n { return sc;}
    else {return lc;}
}

fn p003(mut n: u64) -> u64{
    // n = 600851475143
    let mut i:u64 = 2;
    let mut p:u64 = 1;
    
    while n > 1 {
        while n%i==0 {n/=i;p=i;}
        i+=1;
    }
    return p;
}
*/

fn main() {
   println!("p003: {}", 2); 
}

