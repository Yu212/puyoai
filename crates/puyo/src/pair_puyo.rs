pub struct PairPuyo {
    pub axis: u8,
    pub child: u8,
}

impl PairPuyo {
    pub fn new(axis: u8, child: u8) -> Self {
        Self {
            axis,
            child
        }
    }
}
