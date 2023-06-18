use std::env;
use mayo_lib::cpu::CPU;
use mayo_lib::cpu::instructions::*;
use mayo_lib::cpu::register::Register;
use mayo_lib::devices::memory::Memory;
use mayo_lib::devices::memory_mapper::MemoryMapper;
use mayo_lib::devices::screen_device::ScreenDevice;

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
    // 256 is how many different combinations of 8 bits (a byte) there are (2^8)
    // 256^2 is how many different combinations of two bytes there are
    // same as 2^16 (how many combinations of 16 bits)
    let mut memory = vec![0; 256*256];

    let mut i = 0;
    let mut add = |n: u8| {
        memory[i] = n;
        i += 1;
    };

    let mut write_char = |char: char, command: u8, pos: u8| {
        add(MOV_LIT_REG);
        add(command);
        add(char as u8);
        add(Register::R1 as u8);

        add(MOV_REG_MEM);
        add(Register::R1 as u8);
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

    let mut cpu = CPU::new(Box::new(mm));

    cpu.run();
}

fn normal_mode() {
    todo!()
}
