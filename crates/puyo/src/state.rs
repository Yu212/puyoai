use crate::bit_field::BitField;

pub struct State {
    pub field: [BitField; 5],
}

impl State {
    pub fn new() -> Self {
        Self {
            field: [BitField::new(), BitField::new(), BitField::new(), BitField::new(), BitField::new()],
        }
    }
    pub fn set(&mut self, c: u8, y: usize, x: usize) {
        self.field[c as usize].set(y, x);
    }
    pub fn get(&self, y: usize, x: usize) -> Option<u8> {
        (0..5).find(|&i| self.field[i as usize].get(y, x))
    }
    pub fn erase_one(&mut self) {
        let erase_mask = self.field[0].erase_mask() | self.field[1].erase_mask() | self.field[2].erase_mask() | self.field[3].erase_mask();
        for i in 0..4 {
            self.field[i].erase(erase_mask);
        }
    }
    pub fn drop(&mut self, c: u8, x: usize) {
        let h = self.height(x);
        if h < 13 {
            self.field[c as usize].data[x] |= 1 << h;
        }
    }
    pub fn height(&self, x: usize) -> usize {
        (self.field[0].data[x] | self.field[1].data[x] | self.field[2].data[x] | self.field[3].data[x] | self.field[4].data[x]).trailing_ones() as usize
    }
}
