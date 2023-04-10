
/// Returns true iff the signed value `n` fits into `width` signed bits.
/// 
/// # Arguments:
/// * `n`: A signed integer value
/// * `width`: the width of a bit field
pub fn fitss(n: i64, width: u64) -> bool {
    let max: u128 = 1 << width+1;
    let min = -1*((1 as u128) << width+1) as i128;
    return max as i128 >= n as i128 && n as i128 >= min;
}

/// Returns true iff the unsigned value `n` fits into `width` unsigned bits.
/// 
/// # Arguments:
/// * `n`: An usigned integer value
/// * `width`: the width of a bit field
pub fn fitsu(n: u64, width: u64) -> bool {
    let max: u128 = (1 << width+1) -1;
    return max>=n as u128;
}

/// Retrieve a signed value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn gets(word: u64, width: u64, lsb: u64) -> i64 {
    if ((word<<lsb)>>(62)) as u64 == 1{
        return -1*(word << lsb >> (64 - width)) as i64;
    }
    return (word << lsb >> (64 - width)) as i64;
}

/// Retrieve an unsigned value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn getu(word: u64, width: u64, lsb: u64) -> u64 {
    return (word << lsb >> (64 - width)) as u64;
}

/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the unsigned `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` unsigned bits.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the unsigned value to place into that bit field
pub fn newu(word: u64, width: u64, lsb: u64, value: u64) -> Option<u64> {
    if !fitsu(value, width){
        None
    }else{
        Some((value << (64-lsb-width))|word)
    }
}

/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the signed `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` signed bits.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the signed value to place into that bit field
pub fn news(word: u64, width: u64, lsb: u64, value: i64) -> Option<u64> {
    if !fitss(value, width){
        None
    }else{
        let base = ((1 as i128) << width) -1;
        let num = (((value as i128) & (base as i128)) << (64-lsb-width)) as u64;
        Some(num|word)
    }
}
#[cfg(test)]
mod tests {
    use crate::bitpack::{gets, news, getu, newu, fitsu};

    use super::fitss;

    #[test]
    fn s_fit(){
        assert_eq!(true, fitss(18, 5))
    }
    #[test]
    fn u_fit(){
        assert_eq!(true, fitsu(18, 5))
    }
    #[test]
    fn s_get(){
        let num: i64 = 9;
        assert_eq!(num, gets(18, 4, 59))
    }
    #[test]
    fn u_get(){
        let num: u64 = 9;
        assert_eq!(num, getu(18, 4, 59))
    }
    #[test]
    fn signed() {
        let result: i64 = 18;
        assert_eq!(result, gets(news(0, 5, 12, 18).unwrap(), 5, 12));
    }
    #[test]
    fn unsigned() {
        let result: u64 = 18;
        assert_eq!(result, getu(newu(0, 5, 12, 18).unwrap(), 5, 12));
    }
    #[test]
    fn s_new(){
        let n: u64 = 18;
        assert_eq!(n, news(0, 4, 59, 9).unwrap())
    }
    #[test]
    fn u_new(){
        let n: u64 = 18;
        assert_eq!(n, newu(0, 4, 59, 9).unwrap())
    }
}