pub fn p003(mut n: u64) -> u64{
    // n = 600851475143
    let mut i:u64 = 2;
    let mut p:u64 = 1;
    
    while n > 1 {
        while n%i==0 {n/=i;p=i;}
        i+=1;
    }
    return p;
}

