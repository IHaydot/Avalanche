

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VMStates{
    IDLE = 0,
    NO_PROGRAM = 1,
    ACTIVE = 2,
    BAD_PROGRAM = 3,
    UNITILIZED = 4,
    
}