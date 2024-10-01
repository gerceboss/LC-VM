use crate::registers::Register;
use std::io::Read;

pub enum TrapCode {
    GETC = 0x20,  // 32 - get character from keyboard, not echoed onto the terminal
    OUT = 0x21,   // 33 - output a character
    PUTS = 0x22,  // 34 - output a word string
    IN = 0x23,    // 35 - get character from keyboard, echoed onto the terminal
    PUTSP = 0x24, // 36 - output a byte string
    HALT = 0x25,  // 37 - halt the program
}

// Programs generally start at address 0x3000 only because
// the lower address are left empty for trap routine codes.

// get a single character as input
pub fn trap_getc(reg: &mut Vec<u16>) {
    let mut buffer = [0 as u8; 1];
    std::io::stdin().read_exact(&mut buffer).unwrap();
    reg[Register::R0] = buffer[0].into();
}

pub fn trap_out(reg: &mut Vec<u16>) {
    println!("{}", (reg[Register::R0] as u8) as char);
}

pub fn trap_puts(reg: &mut Vec<u16>, memory: &mut Vec<u16>) {
    let mut index = reg[Register::R0] as usize;

    // NOTE : when casting u16->u8 ,upper 8 bits are truncated
    while index != memory.len() && memory[index] != 0 {
        print!("{}", (memory[index] as u8) as char);
        index += 1;
    }
}

pub fn trap_in(reg: &mut Vec<u16>) {
    println!("Enter a character: ");
    reg[Register::R0] = std::io::stdin()
        .bytes()
        .next()
        .and_then(|res| res.ok())
        .map(|byte| byte as u16)
        .unwrap();
}

pub fn trap_putsp(reg: &mut Vec<u16>, memory: &mut Vec<u16>) {
    let mut index = reg[Register::R0] as usize;

    // NOTE : when casting u16->u8 ,upper 8 bits are truncated
    while index != memory.len() && memory[index] != 0 {
        // a word here is of 16 bits or 2 bytes
        // work gives an array of type u8
        let word = memory[index].to_be_bytes();

        print!("{}", word[1] as char);

        if word[0] != 0 {
            print!("{}", word[0] as char);
        }
        index += 1;
    }
}

// will be used in the main code
pub fn trap_halt(running: &mut bool) {
    println!("HALT Trapcode received, Halting.");
    *running = false;
}
