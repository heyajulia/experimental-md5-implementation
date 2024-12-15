#[cfg(not(feature = "dangerously-enable-md5"))]
compile_error!("This crate has been disabled.");

const S: [u32; 64] = [
    7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9,
    14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10, 15,
    21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
];

pub fn compute(data: &[u8]) -> [u8; 16] {
    let k = (0..64)
        .map(|i| (2_f64.powi(32) * (i as f64 + 1.0).sin().abs()).floor() as u32)
        .collect::<Vec<_>>();

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
            let mut f;
            let g;

            match i {
                0..=15 => {
                    f = (b & c) | ((!b) & d);
                    g = i;
                }
                16..=31 => {
                    f = (d & b) | ((!d) & c);
                    g = (5 * i + 1) % 16;
                }
                32..=47 => {
                    f = b ^ c ^ d;
                    g = (3 * i + 5) % 16;
                }
                48..=63 => {
                    f = c ^ (b | (!d));
                    g = (7 * i) % 16;
                }
                _ => unreachable!(),
            };

            f = f.wrapping_add(a.wrapping_add(k[i]).wrapping_add(m[g]));
            a = d;
            d = c;
            c = b;
            b = b.wrapping_add(f.rotate_left(S[i]));
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
        fn reference_implementation(input in any::<Vec<u8>>()) {
            use md5_reference::Digest;

            let my_hash = compute(&input);
            let reference_hash = md5_reference::Md5::digest(&input);

            prop_assert_eq!(my_hash, &*reference_hash);
        }
    }
}
