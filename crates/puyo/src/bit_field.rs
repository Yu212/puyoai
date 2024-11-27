use std::arch::x86_64::_pext_u32;
use std::simd::u16x8;

pub struct BitField {
    pub data: u16x8,
}

impl BitField {
    pub fn new() -> Self {
        Self {
            data: u16x8::splat(0),
        }
    }
    pub fn set(&mut self, y: usize, x: usize) {
        self.data[x] |= 1 << y;
    }
    pub fn get(&self, y: usize, x: usize) -> bool {
        (self.data[x] >> y & 1) != 0
    }
    pub fn erase(&mut self, mask: u16x8) {
        let v: &mut [u16; 8] = self.data.as_mut();
        for i in 0..6 {
            v[i] = unsafe { _pext_u32(v[i] as u32, !mask[i] as u32) as u16 };
        }
    }
    pub fn erase_mask(&self) -> u16x8 {
        let field12 = self.data & u16x8::from_slice(&[0xfff, 0xfff, 0xfff, 0xfff, 0xfff, 0xfff, 0, 0]);
        let u = field12 >> 1;
        let d = field12 << 1;
        let l = field12.rotate_elements_right::<1>();
        let r = field12.rotate_elements_left::<1>();
        let ud_and = u & d;
        let lr_and = l & r;
        let ud_or = u | d;
        let lr_or = l | r;
        let s3 = ((ud_and & lr_or) | (ud_or & lr_and)) & field12;
        let s2 = (ud_and | lr_and | (ud_or & lr_or)) & field12;
        let s2u = s2 >> 1;
        let s2d = s2 << 1;
        let s2l = s2.rotate_elements_right::<1>();
        let s2r = s2.rotate_elements_left::<1>();
        let seed = s3 | ((s2u | s2d | s2l | s2r) & s2);
        let su = seed >> 1;
        let sd = seed << 1;
        let sl = seed.rotate_elements_right::<1>();
        let sr = seed.rotate_elements_left::<1>();
        (seed | su | sd | sl | sr) & field12
    }
}
