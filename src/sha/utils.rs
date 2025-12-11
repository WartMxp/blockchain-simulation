pub(crate) const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5,
    0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3,
    0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc,
    0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
    0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
    0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3,
    0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5,
    0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
    0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

pub(crate) fn _shb(x: u32, n: u32) -> u32 {
    x >> (n & 31)
}

pub(crate) fn _shw(x: u32, n: u32) -> u32 {
    x << (n & 31)
}

pub(crate) fn _r(x: u32, n: u32) -> u32 {
    x.rotate_right(n)
}

pub(crate) fn _ch(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (!x & z)
}

pub(crate) fn _ma(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (x & z) ^ (y & z)
}

pub(crate) fn _s0(x: u32) -> u32 {
    _r(x, 2) ^ _r(x, 13) ^ _r(x, 22)
}

pub(crate) fn _s1(x: u32) -> u32 {
    _r(x, 6) ^ _r(x, 11) ^ _r(x, 25)
}

pub(crate) fn _g0(x: u32) -> u32 {
    _r(x, 7) ^ _r(x, 18) ^ _shb(x, 3)
}

pub(crate) fn _g1(x: u32) -> u32 {
    _r(x, 17) ^ _r(x, 19) ^ _shb(x, 10)
}

pub(crate) fn _word(c: &[u8]) -> u32 {
    _shw(c[0] as u32, 24) | _shw(c[1] as u32, 16) | _shw(c[2] as u32, 8) | (c[3] as u32)
}

