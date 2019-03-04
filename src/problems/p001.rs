fn seq_sum(n: i32) -> i32{
    return n*(n+1)/2;
}

pub fn p001(mut n: i32) -> i32{
    // n = 1000
    n-=1;
    return 3*seq_sum(n/3)
        +  5*seq_sum(n/5)
        -  15*seq_sum(n/15);
}
