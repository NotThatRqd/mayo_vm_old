use mayo_lib::cpu::CPU;
use mayo_lib::cpu::instructions::*;
use mayo_lib::cpu::register::Register;
use mayo_lib::create_memory::create_memory;
use console::Term;
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
    let mut memory = create_memory(256*256);

    let mut i = 0;
    let mut add = |n: u8| {
        memory[i] = n;
        i += 1;
    };

    let subroutine_address = (0x30, 0x00);

    add(PSH_LIT);
    add(0x33);
    add(0x33);

    add(PSH_LIT);
    add(0x22);
    add(0x22);

    add(PSH_LIT);
    add(0x11);
    add(0x11);

    add(MOV_LIT_REG);
    add(0x12);
    add(0x34);
    add(R1);

    add(MOV_LIT_REG);
    add(0x56);
    add(0x78);
    add(R4);

    add(PSH_LIT);
    add(0x00);
    add(0x00);

    add(CAL_LIT);
    add(subroutine_address.0);
    add(subroutine_address.1);

    add(PSH_LIT);
    add(0x44);
    add(0x44);

    // Subroutine..
    let mut i = 0x3000;
    let mut add = |n: u8| {
        memory[i] = n;
        i += 1;
    };

    add(PSH_LIT);
    add(0x01);
    add(0x02);

    add(PSH_LIT);
    add(0x03);
    add(0x04);

    add(PSH_LIT);
    add(0x05);
    add(0x06);

    add(MOV_LIT_REG);
    add(0x07);
    add(0x08);
    add(R1);

    add(MOV_LIT_REG);
    add(0x09);
    add(0x0A);
    add(R8);

    add(RET);

    let memory = Box::new(Memory::from_vec(memory));

    let mut mm = MemoryMapper::new();
    mm.map(memory, 0, 0xFFFF, true);

    let screen_device = Box::new(ScreenDevice::new());
    mm.map(screen_device, 0x3000, 0x30FF, true);

    let mut cpu = CPU::new(mm);

    loop {
        cpu.step().unwrap();
        cpu.debug();
        println!();
        cpu.view_memory_at(cpu.get_register(Register::Ip) as usize, 8)
            .unwrap();
        //                 the stack
        cpu.view_memory_at(0xFFFF - 1 - 43, 44)
            .unwrap();
        println!();

        // wait for some user input
        Term::stdout().read_char().unwrap();
    }
}
