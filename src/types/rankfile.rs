#[derive(PartialEq, Eq, Ord, PartialOrd)]
pub enum Rank { R1, R2, R3, R4, R5, R6, R7, R8 }

impl Rank {
    pub const NUM: usize = 8;
    pub const fn new(value: u8) -> Self {
        unsafe { std::mem::transmute(value) }
    }

    pub fn index(self) -> usize {
        self as usize
    }
}

#[derive(PartialEq, Eq, Ord, PartialOrd)]
pub enum File {A, B, C, D, E, F, G, H}

impl File {
    pub const fn new(value: u8) -> Self {
        unsafe { std::mem::transmute(value) }
    }
    pub const NUM: usize = 8;
    pub fn index(self) -> usize {
        self as usize
    }
}
