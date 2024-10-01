pub mod registers {
    use std::ops::{Index, IndexMut};

    //LC3 supports 8 general purpose registers
    enum Register {
        R0,
        R1,
        R2,
        R3,
        R4,
        R5,
        R6,
        R7,
        PC,
        COUNT,
        CONDVAR,
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
}

pub mod opcodes {
    use std::ops::{Index, IndexMut};

    // Now for instruction set , instructions supported are of 16 bits
    // We need supported OP_CODES which are of 4 bits
    #[allow(non_camel_case_types)]
    enum OpCode {
        OP_ADD,  // add
        OP_AND,  // bitwise and
        OP_NOT,  // bitwise not
        OP_LD,   // load
        OP_ST,   // store
        OP_JMP,  // jump
        OP_BR,   // branch
        OP_LDR,  // load register
        OP_STR,  // store register
        OP_JSR,  // jump register
        OP_RTI,  // unused
        OP_LDI,  // load indirect
        OP_STI,  // store indirect
        OP_RES,  // reserved (unused)
        OP_LEA,  // load effective address
        OP_TRAP, // execute trap
    }

    // we need indexing for them as well
    impl<T> Index<OpCode> for Vec<T> {
        type Output = T;
        fn index(&self, idx: OpCode) -> &Self::Output {
            &self[idx as usize]
        }
    }

    impl<T> IndexMut<OpCode> for Vec<T> {
        fn index_mut(&mut self, idx: OpCode) -> &mut Self::Output {
            &mut self[idx as usize]
        }
    }
}

// These flags store the sign of previous calculations in LC3
pub mod condflags {
    enum Condflag {
        POS_FL = 1 << 0,  //POSITIVE
        ZERO_FL = 1 << 1, // ZERO
        NEG_FL = 1 << 2,  // NEGATIVE
    }
}
