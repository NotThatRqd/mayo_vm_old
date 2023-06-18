use enum_iterator::{Sequence};

#[derive(Copy, Clone, Debug, Sequence, Eq, PartialEq)]
pub enum Register {
    Ip,
    Acc,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    Sp,
    Fp,
}

impl Register {
    pub fn as_index(&self) -> usize {
        // multiplied by two because registers are two bytes big
        *self as usize * 2
    }
}
