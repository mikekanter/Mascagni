#[derive(PartialEq, Eq, Ord, PartialOrd)]
pub enum Rank { R1, R2, R3, R4, R5, R6, R7, R8 }

pub enum FileError {
    NoFileForByte,
}

impl Rank {
    pub const NUM: usize = 8;
    pub const fn new(value: u8) -> Self {
        unsafe { std::mem::transmute(value) }
    }

    pub fn index(self) -> usize {
        self as usize
    }

    pub fn distance(self, rhs: Rank) -> usize {
        self.index().abs_diff(rhs.index())
    }
}

impl ToString for Rank {
    fn to_string(&self) -> String {
        let s = match self {
            Rank::R1 => "1",
            Rank::R2 => "2",
            Rank::R3 => "3",
            Rank::R4 => "4",
            Rank::R5 => "5",
            Rank::R6 => "6",
            Rank::R7 => "7",
            Rank::R8 => "8",
        };
        s.to_string()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum File {A, B, C, D, E, F, G, H}

impl File {
    pub const fn new(value: u8) -> Self {
        unsafe { std::mem::transmute(value) }
    }
    pub const NUM: usize = 8;
    pub fn index(self) -> usize {
        self as usize
    }

    pub fn distance(self, rhs: File) -> usize {
        self.index().abs_diff(rhs.index())
    }
}

impl ToString for File {
    fn to_string(&self) -> String {
        let s = match self {
            File::A => "a",
            File::B => "b",
            File::C => "c",
            File::D => "d",
            File::E => "e",
            File::F => "f",
            File::G => "g",
            File::H => "h",
        };
        s.to_string()
    }
}

impl TryFrom<u8> for File {
    type Error = FileError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'a' => Ok(File::A),
            b'b' => Ok(File::B),
            b'c' => Ok(File::C),
            b'd' => Ok(File::D),
            b'e' => Ok(File::E),
            b'f' => Ok(File::F),
            b'g' => Ok(File::G),
            b'h' => Ok(File::H),
            _ => Err(FileError::NoFileForByte),
        }
    }
}
