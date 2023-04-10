#[derive(Debug, Default)]
pub struct Registers {
    pub a: u8,
    pub status: Status,
    pub program_counter: u16,
}

#[derive(Debug, Default)]
pub struct Status {
    /// N
    pub negative: bool,
    /// V
    pub overflow: bool,
    /// -
    pub unused: bool,
    /// B
    pub brk: bool,
    /// D
    pub decimal: bool,
    /// I
    pub interrupt_disable: bool,
    /// Z
    pub zero: bool,
    /// C
    pub carry: bool,
}