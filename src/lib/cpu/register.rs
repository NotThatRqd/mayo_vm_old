use enum_iterator::Sequence;

// todo: instead of register being an enum, it should just be a bunch of u8 consts for the index of it
#[derive(Copy, Clone, Debug, Sequence, Hash, PartialEq, Eq)]
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
