use mayo_lib::cpu::CPU;
use mayo_lib::cpu::instructions::*;
use mayo_lib::cpu::register::Register;
use mayo_lib::create_memory::create_memory;

const IP: u8  = 0;
const ACC: u8 = 1;
const R1: u8  = 2;
const R2: u8  = 3;

fn main() {
    let mut memory = create_memory(256*256);

    let mut i = 0;
    let mut add = |n: u8| {
        memory[i] = n;
        i += 1;
    };

    add(MOV_LIT_REG);
    add(0x12);
    add(0x34);
    add(R1);

    add(MOV_LIT_REG);
    add(0xAB);
    add(0xCD);
    add(R2);

    add(ADD_REG_REG);
    add(R1);
    add(R2);

    add(MOV_REG_MEM);
    add(ACC);
    add(0x01);
    add(0x00);


    let mut cpu = CPU::new(memory);

    let mut step = || {
        cpu.step();
        cpu.debug();
        println!();
        cpu.view_memory_at(cpu.get_register(Register::Ip).unwrap() as usize);
        cpu.view_memory_at(0x0100);
        println!();
    };

    step();
    step();
    step();
    step();
}
