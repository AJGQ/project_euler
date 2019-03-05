/*
 * An idea is multiply all biggest prime powers 
 * lower than the upper limit, for example:
 * lcm(1..10) = 2^3*3^2*5*7
 * lcm(1..20) = 2^4*3^2*5*7*11*13*17*19
 *
 */
fn is_prime(n: u64) -> bool {
    if n==2 {return true;}
    if n==1 || n%2==0 {return false;}
    let mut i: u64 = 3;
    while i*i<=n {
        if n%i==0 {return false;}
        i += 1;
    }
    return true;
}

fn biggest_power_until(p: u64, up: u64) -> (u64, u64){
    let mut i : u64 = 0;
    let mut pn: u64 = p;

    while pn<=up {
        pn *= p;
        i += 1;
    }
    return (i,pn/p);
}

pub fn p005(i: u64, f: u64) -> u64 {
    let mut res: u64 = 1;
    for p in i..=f {
        if is_prime(p) {
            res *= biggest_power_until(p,f).1;
        }
    }
    return res;
}
