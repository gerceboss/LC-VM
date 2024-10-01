extern crate termios;

use std::env;
use std::process;
use termios::*;

mod utils;
use utils::*;

mod op_codes;
use op_codes::*;

mod registers;
use registers::*;

mod memory;
use memory::*;

mod trap_codes;
use trap_codes::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Error: provide atleast one VM image");
        println!("Usage: rust-vm <image-file1> [image-file2]..");
        process::exit(2);
    }

    // size of memory in LC3 is 128KB
    let mut memory = vec![0u16; 65536];

    for i in 1..args.len() {
        //yet to write parsing of the file
        if !read_image(&args[i], &mut memory) {
            println!("Failed to load image: {}", args[i]);
            process::exit(1);
        }
    }

    //Platform Specifics (Unix here)
    //Setting terminal input/output behaviour such as accepting
    //character without the need for a newline character
    //Refer: https://stackoverflow.com/questions/26321592/how-can-i-read-one-character-from-stdin-without-having-to-hit-enter
    let stdin = 0;

    let termios = Termios::from_fd(stdin).unwrap();

    let mut new_termios = termios.clone(); // make a mutable copy of termios
                                           // that we will modify
    new_termios.c_iflag &= IGNBRK | BRKINT | PARMRK | ISTRIP | INLCR | IGNCR | ICRNL | IXON;
    new_termios.c_lflag &= !(ICANON | ECHO); // no echo and canonical mode
    tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();

    //Platform specific end

    let _args: Vec<String> = env::args().collect();

    #[allow(non_snake_case)]
    let PC_START: u16 = 0x3000; //default starting address for PC to ensure smooth execution

    //LC3 register
    let mut registers: Vec<u16> = vec![0; Register::COUNT as usize];

    registers[Register::PC] = PC_START; //Default starting address

    let mut running: bool = true;

    while running {
        let ins: u16 = mem_read(registers[Register::PC], &mut memory);

        registers[Register::PC] = registers[Register::PC] + 1; //increment PC

        let op: u16 = ins >> 12; //opcode is in left 4 bits.
                                 //println!("Executing Ins {:#018b} and Opcode bit: {}", ins, op);

        match op {
            op if op == OpCode::OP_BR as u16 => {
                //println!("Executing BRANCH, Ins {:#018b}", ins);
                op_br(&mut registers, ins);
            }

            op if op == OpCode::OP_ADD as u16 => {
                //println!("Executing ADD, Ins {:#018b}", ins);
                op_add(&mut registers, ins);
            }

            op if op == OpCode::OP_LD as u16 => {
                //println!("Executing LD , Ins {:#018b}", ins);
                op_ld(&mut registers, ins, &mut memory);
            }

            op if op == OpCode::OP_ST as u16 => {
                //println!("Executing ST , Ins {:#018b}", ins);
                op_st(&mut registers, ins, &mut memory);
            }

            op if op == OpCode::OP_JSR as u16 => {
                //println!("Executing JSR, Ins {:#018b}", ins);
                op_jsr(&mut registers, ins);
            }

            op if op == OpCode::OP_AND as u16 => {
                //println!("Executing AND, Ins {:#018b}", ins);
                op_and(&mut registers, ins);
            }

            op if op == OpCode::OP_LDR as u16 => {
                //println!("Executing LDR, Ins {:#018b}", ins);
                op_ldr(&mut registers, ins, &mut memory);
            }

            op if op == OpCode::OP_STR as u16 => {
                //println!("Executing STR, Ins {:#018b}", ins);
                op_str(&mut registers, ins, &mut memory);
            }

            op if op == OpCode::OP_RTI as u16 => {
                println!("Bad OpCode 'RTI' received. Aborting.");
                process::exit(10);
            }

            op if op == OpCode::OP_NOT as u16 => {
                //println!("Executing NOT, Ins {:#018b}", ins);
                op_not(&mut registers, ins);
            }

            op if op == OpCode::OP_LDI as u16 => {
                //println!("Executing LDI, Ins {:#018b}", ins);
                op_ldi(&mut registers, ins, &mut memory);
            }

            op if op == OpCode::OP_STI as u16 => {
                //println!("Executing STI, Ins {:#018b}", ins);
                op_sti(&mut registers, ins, &mut memory);
            }

            op if op == OpCode::OP_JMP as u16 => {
                //println!("Executing JMP, Ins {:#018b}", ins);
                op_jmp(&mut registers, ins);
            }

            op if op == OpCode::OP_RES as u16 => {
                println!("Bad OpCode 'RES' received. Aborting.");
                process::exit(10);
            }

            op if op == OpCode::OP_LEA as u16 => {
                //println!("Executing ADD, Ins {:#018b}", ins);
                op_lea(&mut registers, ins);
            }

            //first 4 bits = 1111, is for trap code
            op if op == OpCode::OP_TRAP as u16 => {
                //0xFF = 255, trapcode is identified by the last 8
                //bits of the instruction
                let trap: u16 = ins & 0xFF;
                match trap {
                    trap if trap == TrapCode::GETC as u16 => {
                        //println!("Executing GETC TRAP, Ins {:#018b}", ins);
                        trap_getc(&mut registers);
                    }

                    trap if trap == TrapCode::OUT as u16 => {
                        //println!("Executing OUT TRAP, Ins {:#018b}", ins);
                        trap_out(&mut registers);
                    }

                    trap if trap == TrapCode::PUTS as u16 => {
                        //println!("Executing PUTS TRAP, Ins {:#018b}", ins);
                        trap_puts(&mut registers, &mut memory);
                    }

                    trap if trap == TrapCode::IN as u16 => {
                        //println!("Executing IN  TRAP, Ins {:#018b}", ins);
                        trap_in(&mut registers);
                    }

                    trap if trap == TrapCode::PUTSP as u16 => {
                        //println!("Executing PUTSP TRAP, Ins {:#018b}", ins);
                        trap_putsp(&mut registers, &mut memory);
                    }

                    trap if trap == TrapCode::HALT as u16 => {
                        trap_halt(&mut running);
                    }

                    _ => {
                        println!("Invalid Trap Code received, aborting.");
                        process::exit(21);
                    }
                }
            }

            _ => {
                println!("Invalid Opcode recieved, aborting current image.");
                process::exit(20);
            }
        }
    }

    // reset the stdin to original termios data

    tcsetattr(stdin, TCSANOW, &termios).unwrap();
    println!("Shutting Down VM...");
}
