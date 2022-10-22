/// FNV1A hash function.
/// https://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function
pub fn fnv1a_32<'a>(bytes: &'a [u8]) -> u32 {
    let mut hash: u32 = 2166136261;
    let prime = 16777619;

    for b in bytes {
        hash ^= *b as u32;
        hash = hash.wrapping_mul(prime);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let bytes = "hello world!".as_bytes();
        assert_eq!(2956263410, fnv1a_32(bytes))
    }

    #[test]
    fn case2() {
        let bytes = "foo bar!".as_bytes();
        assert_eq!(1510564049, fnv1a_32(bytes))
    }

    #[test]
    fn case3() {
        let bytes = "herp".as_bytes();
        assert_eq!(876315572, fnv1a_32(bytes))
    }
}
