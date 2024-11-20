use pyo3::prelude::*;

#[inline]
fn load64le(p: &[u8]) -> u64 {
    u64::from(p[0]) << 0 | u64::from(p[1]) << 8 |
    u64::from(p[2]) << 16 | u64::from(p[3]) << 24 |
    u64::from(p[4]) << 32 | u64::from(p[5]) << 40 |
    u64::from(p[6]) << 48 | u64::from(p[7]) << 56
}

#[pyfunction]
fn chibihash64(key: &[u8], seed: u64) -> u64 {
    let mut k = key;
    let mut l = key.len() as isize;

    const P1: u64 = 0x2B7E151628AED2A5;
    const P2: u64 = 0x9E3793492EEDC3F7;
    const P3: u64 = 0x3243F6A8885A308D;

    let mut h = [P1, P2, P3, seed];

    while l >= 32 {
        for i in 0..4 {
            let lane = load64le(&k[i*8..]);
            h[i] ^= lane;
            h[i] = h[i].wrapping_mul(P1);
            h[(i+1)&3] ^= (lane << 40) | (lane >> 24);
        }
        k = &k[32..];
        l -= 32;
    }

    h[0] = h[0].wrapping_add((key.len() as u64) << 32 | (key.len() as u64) >> 32);
    if l & 1 != 0 {
        h[0] ^= u64::from(k[0]);
        l -= 1;
        k = &k[1..];
    }
    h[0] = h[0].wrapping_mul(P2);
    h[0] ^= h[0] >> 31;

    let mut i = 1;
    while l >= 8 {
        h[i] ^= load64le(k);
        h[i] = h[i].wrapping_mul(P2);
        h[i] ^= h[i] >> 31;
        k = &k[8..];
        l -= 8;
        i += 1;
    }

    i = 0;
    while l > 0 {
        h[i] ^= u64::from(k[0]) | (u64::from(k[1]) << 8);
        h[i] = h[i].wrapping_mul(P3);
        h[i] ^= h[i] >> 31;
        k = &k[2..];
        l -= 2;
        i += 1;
    }

    let mut x = seed;
    x ^= h[0].wrapping_mul((h[2] >> 32)|1);
    x ^= h[1].wrapping_mul((h[3] >> 32)|1);
    x ^= h[2].wrapping_mul((h[0] >> 32)|1);
    x ^= h[3].wrapping_mul((h[1] >> 32)|1);

    x ^= x >> 27;
    x = x.wrapping_mul(0x3C79AC492BA7B653);
    x ^= x >> 33;
    x = x.wrapping_mul(0x1C69B3F74AC4AE35);
    x ^= x >> 27;

    x
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(chibihash64, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_reference_values() {
        assert_eq!(chibihash64(&[], 0), 0x9EA80F3B18E26CFB);
        assert_eq!(chibihash64(&[], 55555), 0x2EED9399FC4AC7E5);
        assert_eq!(chibihash64(b"hi", 0), 0xAF98F3924F5C80D6);
        assert_eq!(chibihash64(b"123", 0), 0x893A5CCA05B0A883);
        assert_eq!(chibihash64(b"abcdefgh", 0), 0x8F922660063E3E75);
        assert_eq!(chibihash64(b"Hello, world!", 0), 0x5AF920D8C0EBFE9F);
        assert_eq!(chibihash64(b"qwertyuiopasdfghjklzxcvbnm123456", 0), 0x2EF296DB634F6551);
        assert_eq!(chibihash64(b"qwertyuiopasdfghjklzxcvbnm123456789", 0), 0x0F56CF3735FFA943);
    }

    #[test]
    fn test_same_string_same_hash() {
        let s = b"test string";
        assert_eq!(chibihash64(s, 0), chibihash64(s, 0));
    }

    #[test]
    fn test_different_strings_different_hashes() {
        assert_ne!(chibihash64(b"hello", 0), chibihash64(b"hello!", 0));
    }

    #[test]
    fn test_with_seed() {
        let s = b"test";
        assert_ne!(chibihash64(s, 0), chibihash64(s, 42));
    }

    #[test]
    fn test_long_string() {
        let long_str = vec![b'a'; 100];
        let hash_val = chibihash64(&long_str, 0);
        assert!(hash_val > 0);
    }

    #[test]
    fn test_binary_data() {
        let data: Vec<u8> = (0..32).collect();
        let hash_val = chibihash64(&data, 0);
        assert!(hash_val > 0);
    }
}
