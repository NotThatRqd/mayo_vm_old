use std::env;
use mayo_lib::cpu::CPU;
use mayo_lib::cpu::instructions::*;
use mayo_lib::create_memory::create_memory;
use mayo_lib::devices::memory::Memory;
use mayo_lib::devices::memory_mapper::MemoryMapper;
use mayo_lib::devices::screen_device::ScreenDevice;

const IP: u8  = 0;
const ACC: u8 = 1;
const R1: u8  = 2;
const R2: u8  = 3;
const R3: u8  = 4;
const R4: u8  = 5;
const R5: u8  = 6;
const R6: u8  = 7;
const R7: u8  = 8;
const R8: u8  = 9;
const SP: u8 = 10;
const FP: u8 = 11;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // a cli arg was passed to the program
        let arg1 = &args[1];

        if arg1 == "true" {
            // jump into test mode instead of usual loader
            test_mode();
        } else {
            normal_mode();
        }
    } else {
        // no cli args were passed to the program
        normal_mode();
    }
}

fn test_mode() {
    let mut memory = create_memory(256*256);

    let mut i = 0;
    let mut add = |n: u8| {
        memory[i] = n;
        i += 1;
    };

    let mut write_char = |char: char, command: u8, pos: u8| {
        add(MOV_LIT_REG);
        add(command);
        add(char as u8);
        add(R1);

        add(MOV_REG_MEM);
        add(R1);
        add(0x30);
        add(pos);
    };

    // Clear screen
    write_char(' ', 0xFF, 0);

    for (i, char) in "Hi world!".chars().into_iter().enumerate() {
        let command = if i % 2 == 0 {
            0x01
        } else {
            0x02
        };

        write_char(char, command, i as u8);
    }

    add(HLT);


    let memory = Box::new(Memory::from_vec(memory));

    let mut mm = MemoryMapper::new();
    mm.map(memory, 0, 0xFFFF, true);

    let screen_device = Box::new(ScreenDevice::new());
    mm.map(screen_device, 0x3000, 0x30FF, true);

    let mut cpu = CPU::new(mm);

    cpu.run();
}

fn normal_mode() {
    todo!()
}
