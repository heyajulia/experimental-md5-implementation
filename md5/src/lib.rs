#[cfg(not(feature = "dangerously-enable-md5"))]
compile_error!("This crate has been disabled.");

const S: [u32; 64] = [
    7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9,
    14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10, 15,
    21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
];

const K: [u32; 64] = [
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
];

pub fn compute(data: &[u8]) -> [u8; 16] {
    let mut a0 = 0x67452301_u32;
    let mut b0 = 0xefcdab89_u32;
    let mut c0 = 0x98badcfe_u32;
    let mut d0 = 0x10325476_u32;

    let mut message = data.to_vec();
    let original_length = message.len();
    message.push(0x80);

    while message.len() % 64 != 56 {
        message.push(0x0);
    }

    let original_length_in_bits = (original_length * 8) as u64;
    message.extend_from_slice(&original_length_in_bits.to_le_bytes());

    for i in (0..(message.len())).step_by(64) {
        let block = &message[i..i + 64];
        let mut m: [u32; 16] = [0; 16];

        for (j, chunk) in block.chunks_exact(4).enumerate() {
            m[j] = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        }

        let mut a = a0;
        let mut b = b0;
        let mut c = c0;
        let mut d = d0;

        for i in 0..=63 {
            let mut f = 0;
            let mut g = 0;

            if (0..=15).contains(&i) {
                f = (b & c) | ((!b) & d);
                g = i;
            } else if (16..=31).contains(&i) {
                f = (d & b) | ((!d) & c);
                g = (5 * i + 1) % 16;
            } else if (32..=47).contains(&i) {
                f = b ^ c ^ d;
                g = (3 * i + 5) % 16;
            } else if (48..=63).contains(&i) {
                f = c ^ (b | (!d));
                g = (7 * i) % 16;
            }

            f = f.wrapping_add(a.wrapping_add(K[i]).wrapping_add(m[g]));
            a = d;
            d = c;
            c = b;
            b = b.wrapping_add(leftrotate(f, S[i]));
        }

        a0 = a0.wrapping_add(a);
        b0 = b0.wrapping_add(b);
        c0 = c0.wrapping_add(c);
        d0 = d0.wrapping_add(d);
    }

    let mut digest = [0u8; 16];

    digest[0..4].copy_from_slice(&a0.to_le_bytes());
    digest[4..8].copy_from_slice(&b0.to_le_bytes());
    digest[8..12].copy_from_slice(&c0.to_le_bytes());
    digest[12..16].copy_from_slice(&d0.to_le_bytes());

    digest
}

fn leftrotate(x: u32, n: u32) -> u32 {
    (x << n) | (x >> (32 - n))
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;
    use proptest::prelude::*;

    macro_rules! test {
        ($name:ident, $input:expr, $expected:expr) => {
            #[test]
            fn $name() {
                assert_eq!(compute($input), hex!($expected));
            }
        };
    }

    test!(
        pangram,
        b"The quick brown fox jumps over the lazy dog",
        "9e107d9d372bb6826bd81d3542a419d6"
    );

    test!(
        pangram_period,
        b"The quick brown fox jumps over the lazy dog.",
        "e4d909c290d0fb1ca068ffaddf22cbd0"
    );

    // Test cases borrowed from RFC 1321.

    test!(empty, b"", "d41d8cd98f00b204e9800998ecf8427e");

    test!(a, b"a", "0cc175b9c0f1b6a831c399e269772661");

    test!(abc, b"abc", "900150983cd24fb0d6963f7d28e17f72");

    test!(
        ascii_lowercase,
        b"abcdefghijklmnopqrstuvwxyz",
        "c3fcd3d76192e4007dfb496cca67e13b"
    );

    test!(
        ascii_letters_and_digits,
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
        "d174ab98d277d9f5a5611c2c9f419d9f"
    );

    test!(
        digits_eight_times,
        b"12345678901234567890123456789012345678901234567890123456789012345678901234567890",
        "57edf4a22be3c955ac49da2e2107b67a"
    );

    proptest! {
        #[test]
        fn test_against_reference_implementation(input in any::<Vec<u8>>()) {
            use md5_reference::Digest;

            let my_hash = compute(&input);
            let reference_hash = md5_reference::Md5::digest(&input);

            prop_assert_eq!(my_hash, &*reference_hash);
        }
    }
}
