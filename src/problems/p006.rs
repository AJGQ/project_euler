/*
 * 1+...+n      = n*(n+1)/2
 * 1^2+...+n^2  = n*(n+1)*(2*n+1)/6
 *
 */




pub fn p006(n: u64) -> u64 {
    // n = 100
    return ((n*n*(n+1)*(n+1))/4)-((n*(n+1)*(2*n+1))/6);
}