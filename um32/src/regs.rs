use crate::memory::Platter;

#[derive(Default)]
pub struct Registers {
    pub regs: [Platter; 8],
    pub finger: Platter,
}
