pub fn p002(n: i64) -> i64{
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

