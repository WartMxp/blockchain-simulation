use super::utils::*;


fn _addbits(ctx: &mut SHA256Context, n: u32) {

    if ctx.bits[0] > 0xffff_ffff - (n as u64) {
        ctx.bits[1] = ctx.bits[1].wrapping_add(1) & 0xffff_ffff_ffff_ffff;
    }
    ctx.bits[0] = ctx.bits[0].wrapping_add(n as u64) & 0xffff_ffff_ffff_ffff;

}

fn _hash(ctx: &mut SHA256Context) {

    let mut a = ctx.hash[0];
    let mut b = ctx.hash[1];
    let mut c = ctx.hash[2];
    let mut d = ctx.hash[3];
    let mut e = ctx.hash[4];
    let mut f = ctx.hash[5];
    let mut g = ctx.hash[6];
    let mut h = ctx.hash[7];

    for i in 0..64 {

        let t1 = h
            .wrapping_add(_s1(e))
            .wrapping_add(_ch(e, f, g))
            .wrapping_add(K[i])
            .wrapping_add(ctx.w[i]);
        let t2 = _s0(a).wrapping_add(_ma(a, b, c));
        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(t1);
        d = c;
        c = b;
        b = a;
        a = t1.wrapping_add(t2);

    }

    for i in 0..8 {
        ctx.hash[i] = ctx.hash[i].wrapping_add(match i {
            0 => a,
            1 => b,
            2 => c,
            3 => d,
            4 => e,
            5 => f,
            6 => g,
            7 => h,
            _ => 0,
        });
    }

}

#[derive(Debug, Clone)]
pub(crate) struct SHA256Context {

    pub buf: [u8; 64],
    pub hash: [u32; 8],
    pub bits: [u64; 2],
    pub len: u64,
    pub rfu__: u64,
    pub w: [u32; 64],

}

impl SHA256Context {

    pub(crate) fn init(&mut self) {
        
        self.bits[0] = 0;
        self.bits[1] = 0;
        self.len = 0;
        self.hash[0] = 0x6a09e667;
        self.hash[1] = 0xbb67ae85;
        self.hash[2] = 0x3c6ef372;
        self.hash[3] = 0xa54ff53a;
        self.hash[4] = 0x510e527f;
        self.hash[5] = 0x9b05688c;
        self.hash[6] = 0x1f83d9ab;
        self.hash[7] = 0x5be0cd19;
        
    }

    pub(crate) fn shash(&mut self, data: &[u8]) {

        for &byte in data {
            self.buf[(self.len % 64) as usize] = byte;
            self.len += 1;

            if self.len % 64 == 0 {
                for j in 0..16 {
                    self.w[j] = _word(&self.buf[(j * 4)..(j * 4 + 4)]);
                }
                for j in 16..64 {
                    self.w[j] = _g1(self.w[j - 2])
                        .wrapping_add(self.w[j - 7])
                        .wrapping_add(_g0(self.w[j - 15]))
                        .wrapping_add(self.w[j - 16]);
                }
                _hash(self);
                _addbits(self, 512);
            }
        }

    }

    pub(crate) fn sdone(&mut self, hash: &mut [u32; 8]) -> String {

        let bit_len: u64 = self.len * 8;

        self.buf[(self.len % 64) as usize] = 0x80;
        self.len += 1;

        if self.len % 64 > 56 {
            while self.len % 64 != 0 {
                self.buf[(self.len % 64) as usize] = 0;
                self.len += 1;
            }
            for j in 0..16 {
                self.w[j] = _word(&self.buf[(j * 4)..(j * 4 + 4)]);
            }
            for j in 16..64 {
                self.w[j] = _g1(self.w[j - 2])
                    .wrapping_add(self.w[j - 7])
                    .wrapping_add(_g0(self.w[j - 15]))
                    .wrapping_add(self.w[j - 16]);
            }
            _hash(self);
            self.len = 0;
        }

        while self.len % 64 != 56 {
            self.buf[(self.len % 64) as usize] = 0;
            self.len += 1;
        }

        self.buf[56] = ((bit_len >> 56) & 0xff) as u8;
        self.buf[57] = ((bit_len >> 48) & 0xff) as u8;
        self.buf[58] = ((bit_len >> 40) & 0xff) as u8;
        self.buf[59] = ((bit_len >> 32) & 0xff) as u8;
        self.buf[60] = ((bit_len >> 24) & 0xff) as u8;
        self.buf[61] = ((bit_len >> 16) & 0xff) as u8;
        self.buf[62] = ((bit_len >> 8) & 0xff) as u8;
        self.buf[63] = (bit_len & 0xff) as u8;

        for j in 0..16 {
            self.w[j] = _word(&self.buf[(j * 4)..(j * 4 + 4)]);
        }
        for j in 16..64 {
            self.w[j] = _g1(self.w[j - 2])
                .wrapping_add(self.w[j - 7])
                .wrapping_add(_g0(self.w[j - 15]))
                .wrapping_add(self.w[j - 16]);
        }
        _hash(self);

        for i in 0..8 {
            hash[i] = self.hash[i];
        }

        let mut digest = String::with_capacity(8 * 8);
        for &h in hash.iter() {
            digest.push_str(&format!("{:08x}", h));
        }
        digest

    }

}