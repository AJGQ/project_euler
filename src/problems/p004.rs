/*
 * abc and def, numbers near 1000
 * 
 *  (100a+10b+c)(100d+10e+f) =  10000ad+
 *                              1000(ae+bd)+
 *                              100(af+cd+be)+
 *                              10(bf+ce)+
 *                              cf
 * 
 * a and d, are close to 9
 * so, the palindrome number we are 
 * looking for is probably 6 digits long
 * let's consider that...
 *
 * ghiihg is our palindrome number,
 * that means ghiihg is equal to 
 *
 * g*100001+h*10010+i*1100 = m*n
 * <=>
 * 11*(g*9091+h*910+i*100) = m*n
 * => {m = 11*m'}
 * 9 < m' < 91 and 99 < n < 1000 and 99999 < 11*m'*n < 1000000
 * => 
 * 9 < m' < 91 and 99 < n < 1000 and 9090 < m'*n < 90910
 * =>
 * 9 < m' < 91 
 * =>
 * 9 < m' < 91 
 * =>
 * 9 < m' < 91 
 *
 *
 * 9091 is prime
 * 910 = 2*5*7*13
 * 100 = 2^2*5^2
 *
 * if g == 2 or 5, it means that ghiihg 
 * is multiple of 2 or 5, but that doesn't 
 * help us, I think ...
 * 
 * m' = 83 => 99999/(11*m') < n < 1000000/(11*m') 
 */
/*
fn is_palindrome(n: i32) -> bool{
    let digits: String = n.to_string();
    let digits: &[u8]  = digits.as_bytes();
    let len = digits.len();
    for i in 0..=(len/2) {
        if digits[i] != digits[(len-1)-i] {
            return false;
        }
    }
    
    true
}
*/
pub fn p004() -> i32 {
    let mut num: i32;

    for g in 0..9 {
        for h in 0..=9 {
            for i in 0..=9 {
                num = 100001*(9-g)+10010*(9-h)+1100*(9-i);
                for m in 10..=90 {
                    let n = num/(m*11);
                    if num%(m*11)==0 && 99<n && n<1000 {
                        return num;
                    }
                } 
            }
        } 
    }
    0
}
