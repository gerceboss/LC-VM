// Total memory locations will be 65536

//Memory Mapped Registers
//These special register has addressess reserved for them in memory. So
//to read and write to this register, we read/write into the memory.
//KBSR identifies whether a key was pressed.
//KBDR tells us what key was pressed
#[allow(non_camel_case_types)]
pub enum MemMapReg {
    MR_KBSR = 0xFE00, //Keyboard Status Register. 0xFE00 = 65024.
    MR_KBDR = 0xFE02, //Keyboard Data Register. 0xFE02 = 65026.
}

pub fn mem_read(memory: &mut Vec<u16>, address: u16) -> u16 {
    //let instr: u16 = 0b1111_0000_00100100;
    if address == MemMapReg::MR_KBSR as u16 {
        let mut buffer = [0; 1];
        std::io::stdin().read_exact(&mut buffer).unwrap();

        if buffer[0] != 0 {
            memory[MemMapReg::MR_KBSR as usize] = 1 << 15;
            memory[MemMapReg::MR_KBDR as usize] = buffer[0] as u16;
        } else {
            memory[MemMapReg::MR_KBSR as usize] = 0;
        }
    }

    memory[address as usize]
}

pub fn mem_write(memory: &mut Vec<u16>, address: u16, val: u16) {
    memory[address as usize] = val;
}
