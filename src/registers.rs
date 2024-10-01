use std::ops::{Index, IndexMut};

//LC3 supports 8 general purpose registers
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    PC,
    CONDVAR,
    COUNT,
}

// We need enums as `usize` or numeric values while accessing using vectors or arrays
// So we implement `Index` and `IndexMut`
impl<T> Index<Register> for Vec<T> {
    type Output = T;
    fn index(&self, reg: Register) -> &T {
        &self[reg as usize]
    }
}
impl<T> IndexMut<Register> for Vec<T> {
    fn index_mut(&mut self, reg: Register) -> &mut Self::Output {
        &mut self[reg as usize]
    }
}

// These flags store the sign of previous calculations in LC3
#[allow(non_camel_case_types)]
pub enum Condflag {
    POS_FL = 1 << 0,  //POSITIVE
    ZERO_FL = 1 << 1, // ZERO
    NEG_FL = 1 << 2,  // NEGATIVE
}
