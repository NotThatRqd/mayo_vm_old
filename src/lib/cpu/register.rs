use enum_iterator::Sequence;

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
}
