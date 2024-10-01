//Memory Mapped Registers
//These special register has address reserved for them in memory. So
//to read and write to this register, we read/write into the memory.
//KBSR identifies whether a key was pressed. KBDR tells us what key was
//pressed
#[allow(non_camel_case_types)]
pub enum MemMapReg {
    MR_KBSR = 0xFE00, //Keyboard Status Register. 0xFE00 = 65024.
    MR_KBDR = 0xFE02, //Keyboard Data Register. 0xFE02 = 65026.
}
