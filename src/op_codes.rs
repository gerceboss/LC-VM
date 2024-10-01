use crate::memory::*;
use vm::registers::Register;
use vm::{extend_with_sign, update_flags};
pub fn op_add(reg: &mut Vec<u16>, ins: u16) {
    let r0: usize = ((ins >> 9) & 0x07).into(); // destination register
    let r1: usize = ((ins >> 6) & 0x07).into(); // first operand register
    let imm_flag: u16 = (ins >> 5) & 0x01; // if immediate mode is on

    if imm_flag == 1 {
        //extract last 5 bits of ins, which is the imm number
        //and also extend it
        let imm5: u16 = extend_with_sign(5, ins & 0x1f);
        reg[r0] = u16::wrapping_add(reg[r1], imm5);
    } else {
        let r2: usize = (ins & 0x07).into(); // second operand
        reg[r0] = u16::wrapping_add(reg[r1], reg[r2]);
    }

    //after the addition update the flags

    update_flags(reg, r0);
}

pub fn op_and(reg: &mut Vec<u16>, ins: u16) {
    let r0: usize = ((ins >> 9) & 0x07).into();
    let r1: usize = ((ins >> 6) & 0x07).into();
    let imm_flag: u16 = (ins >> 5) & 0x01;

    if imm_flag == 1 {
        let imm5: u16 = extend_with_sign(5, ins & 0x1f);
        reg[r0] = reg[r1] & imm5;
    } else {
        let r2: usize = (ins & 0x7).into();
        reg[r0] = reg[r1] & reg[r2];
    }

    update_flags(reg, r0);
}

pub fn op_not(reg: &mut Vec<u16>, ins: u16) {
    let r0: usize = ((ins >> 9) & 0x07).into();
    let r1: usize = ((ins >> 6) & 0x07).into();

    reg[r0] = !reg[r1];
    update_flags(reg, r0);
}

pub fn op_ld(reg: &mut Vec<u16>, ins: u16, memory: &mut Vec<u16>) {
    let r0: usize = ((ins >> 9) & 0x7).into();
    let pc_offset: u16 = extend_with_sign(9, ins & 0x1FF);

    reg[r0] = mem_read(u16::wrapping_add(reg[Register::PC], pc_offset), memory);
    update_flags(reg, r0);
}

pub fn op_st(reg: &mut Vec<u16>, ins: u16, memory: &mut Vec<u16>) {
    let r0: usize = ((ins >> 9) & 0x7).into();
    let pc_offset: u16 = extend_with_sign(ins & 0x1FF, 9);
    mem_write(
        u16::wrapping_add(reg[Register::PC], pc_offset),
        memory,
        reg[r0],
    );
}

pub fn op_jmp(reg: &mut Vec<u16>, ins: u16) {
    let r1: usize = ((ins >> 6) & 0x07).into();
    reg[Register::PC] = reg[r1];
}

pub fn op_br(reg: &mut Vec<u16>, ins: u16) {
    let pc_offset: u16 = extend_with_sign(9, ins & 0x1FF);
    let cond_flag: u16 = (ins >> 9) & 0x7;

    // reg[Register::COND] can be either 1, 2, 4 denoting
    // Zero, Negative, Positive
    // If cond_flag matches the condition set in reg[Register::CONDVAR]
    // Then we add pc_offset to current PC and branch of to those insuctions.
    if (cond_flag & reg[Register::CONDVAR]) > 0 {
        reg[Register::PC] = u16::wrapping_add(reg[Register::PC], pc_offset);
    }
}

pub fn op_ldr(reg: &mut Vec<u16>, ins: u16, memory: &mut Vec<u16>) {
    let r0: usize = ((ins >> 9) & 0x7).into();
    let r1: usize = ((ins >> 6) & 0x7).into();
    let offset: u16 = extend_with_sign(6, ins & 0x3F);

    reg[r0] = mem_read(u16::wrapping_add(reg[r1], offset), memory);
    update_flags(reg, r0);
}

pub fn op_str(reg: &mut Vec<u16>, ins: u16, memory: &mut Vec<u16>) {
    let r0: usize = ((ins >> 9) & 0x7).into();
    let r1: usize = ((ins >> 6) & 0x7).into();

    let offset: u16 = extend_with_sign(6, ins & 0x3F);

    mem_write(u16::wrapping_add(reg[r1], offset), memory, reg[r0]);
}

pub fn op_jsr(reg: &mut Vec<u16>, ins: u16) {
    let long_flag: u16 = (ins >> 11) & 1;
    //We save the incremented PC to Register 7 as this
    //helps in allowing us to go back to the sub-routine
    //that initially called and resume the work
    reg[Register::R7] = reg[Register::PC];

    if long_flag == 1 {
        let long_pc_offset = extend_with_sign(11, ins & 0x7FF);
        reg[Register::PC] = u16::wrapping_add(reg[Register::PC], long_pc_offset);
    } else {
        let r1: usize = ((ins >> 6) & 0x07).into();
        reg[Register::PC] = reg[r1];
    }
}

pub fn op_ldi(reg: &mut Vec<u16>, ins: u16, memory: &mut Vec<u16>) {
    let r0: usize = ((ins >> 9) & 0x07).into();
    let pc_offset: u16 = extend_with_sign(9, ins & 0x01FF).into();

    // Interesting fact: If we had reversed the order of arguments it wil not work as
    //it will require memory's mutable refernce but it will be borrowed in the outside `mem_read`` function
    reg[r0] = mem_read(
        mem_read(u16::wrapping_add(reg[Register::PC], pc_offset), memory),
        memory,
    );
    update_flags(reg, r0);
}

pub fn op_sti(reg: &mut Vec<u16>, ins: u16, memory: &mut Vec<u16>) {
    let r0: usize = ((ins >> 9) & 0x7).into();
    let pc_offset: u16 = extend_with_sign(9, ins & 0x1FF);
    mem_write(
        mem_read(u16::wrapping_add(reg[Register::PC], pc_offset), memory),
        memory,
        reg[r0],
    );
}

pub fn op_lea(reg: &mut Vec<u16>, ins: u16) {
    let r0: usize = ((ins >> 9) & 0x7).into();
    let pc_offset: u16 = extend_with_sign(9, ins & 0x1FF);

    reg[r0] = u16::wrapping_add(reg[Register::PC], pc_offset); //lea differs from load in this line
    update_flags(reg, r0);
}
