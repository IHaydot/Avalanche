

#[derive(Clone, Debug)]
#[repr(u64)]
pub enum VMInstructions{
    NOP = 0, //0A
    ADD = 1, //2A result: RR0
    SET = 2, //2A because we dont need to change endian type
    CPY = 3, //2A 1-Copy 2-Paste
    JMP = 4, //1A
    JCS = 5, //1A Jump if SR0 set
    EQ = 6, //2A registers Sets SR0 if eq
    JCNS = 7, //1A
    CALL = 8, //1A
    RET = 9, //0A
    STOP = 10, //0A
    SUB = 11, //2A
    ZERO = 12, //1A
    RCS = 13, //0A
    RCNS = 14, //0A
    ADD1 = 15, //2A ADD with number

    NUL = 0xde
}

impl From<u8> for VMInstructions{
    fn from(u: u8) -> VMInstructions{
        match u{
            0 => Self::NOP,
            1 => Self::ADD,
            2 => Self::SET,
            3 => Self::CPY,
            4 => Self::JMP,
            5 => Self::JCS,
            6 => Self::EQ,
            7 => Self::JCNS,
            8 => Self::CALL,
            9 => Self::RET,
            10 => Self::STOP,
            11 => Self::SUB,
            12 => Self::ZERO,
            13 => Self::RCS,
            14 => Self::RCNS,
            15 => Self::ADD1,

            _ => Self::NUL
        }
    }
}

impl From<i64> for VMInstructions{
    fn from(u: i64) -> VMInstructions{
        match u{
            0 => Self::NOP,
            1 => Self::ADD,
            2 => Self::SET,
            3 => Self::CPY,
            4 => Self::JMP,
            5 => Self::JCS,
            6 => Self::EQ,
            7 => Self::JCNS,
            8 => Self::CALL,
            9 => Self::RET,
            10 => Self::STOP,
            11 => Self::SUB,
            12 => Self::ZERO,
            13 => Self::RCS,
            14 => Self::RCNS,
            15 => Self::ADD1,

            _ => Self::NUL
        }
    }
}
