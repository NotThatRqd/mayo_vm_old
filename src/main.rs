use crate::cpu::{ADD_REG_REG, CPU, MOV_LIT_R1, MOV_LIT_R2};
use crate::create_memory::create_memory;

mod cpu;
mod create_memory;

fn main() {
    let mut memory = create_memory(256);

    memory[0] = MOV_LIT_R1;
    memory[1] = 0x12;
    memory[2] = 0x34;

    memory[3] = MOV_LIT_R2;
    memory[4] = 0xAB;
    memory[5] = 0xCD;

    memory[6] = ADD_REG_REG;
    memory[7] = 0x02; // r1
    memory[8] = 0x03; // r2


    let mut cpu = CPU::new(memory);

    cpu.step();
    cpu.debug();
    println!();

    cpu.step();
    cpu.debug();
    println!();

    cpu.step();
    cpu.debug();
    println!();
}
