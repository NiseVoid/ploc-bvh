fn split(mut x: usize, log_bits: usize) -> usize {
    let bit_count = 1 << log_bits;
    let mut mask = (1 << bit_count) - 1;
    x &= mask;
    let mut n = 1 << (log_bits);
    for _ in (1..=(log_bits)).rev() {
        mask = (mask | (mask << n)) & !(mask << (n / 2));
        x = (x | (x << n)) & mask;
        n >>= 1;
    }
    x
}

pub fn morton_encode(x: usize, y: usize, z: usize, log_bits: usize) -> usize {
    split(x, log_bits) | (split(y, log_bits) << 1) | (split(z, log_bits) << 2)
}

pub const MORTON_CENTER: f32 = (u16::MAX / 2) as f32;

#[test]
fn test_morton_encode() {
    let (x, y, z) = (6.7, 19.3, 2.);
    let morton = morton_encode(x as usize, y as usize, z as usize, 3);
    // 6 =  00110
    // 19 = 10011
    // 2 =  00010
    // Bits should return as zyx_zyx_zyx
    assert_eq!(morton, 0b010_000_001_111_010);

    let (x, y, z) = (6000, 3000, 1234);
    let morton = morton_encode(x as usize, y as usize, z as usize, 4);
    // 6000 = 1011101110000
    // 3000 = 0101110111000
    // 1234 = 0010011010010
    assert_eq!(morton, 0b001_010_101_011_011_110_101_011_111_010_000_100_000);
}
