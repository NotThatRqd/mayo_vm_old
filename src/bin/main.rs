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

    let mut cpu = CPU::new(mm);

    cpu.run();
}

fn normal_mode() {
    todo!()
}

#[cfg(test)]
mod tests {
    use mayo_lib::cpu::CPU;
    use mayo_lib::cpu::instructions::*;
    use mayo_lib::cpu::register::Register;
    use mayo_lib::devices::memory::Memory;

    #[test]
    fn addition_program() {
        let mut mem: Vec<u8> = vec![0; 16];

        let mut i = 0;
        let mut add = |n: u8| {
            mem[i] = n;
            i += 1;
        };

        add(MOV_LIT_REG);
        add(0x12);
        add(0x34);
        add(Register::R1 as u8);

        add(MOV_LIT_REG);
        add(0xAB);
        add(0xCD);
        add(Register::R2 as u8);

        add(ADD_REG_REG);
        add(Register::R1 as u8);
        add(Register::R2 as u8);

        add(HLT);

        let mem = Memory::from_vec(mem);
        let mut cpu = CPU::new(mem);

        cpu.run();

        let acc_value = cpu.get_register(Register::Acc);
        assert_eq!(acc_value, 0x1234 + 0xABCD);
    }

    #[test]
    fn subroutine_program() {
        let mut mem: Vec<u8> = vec![0; 256*256];

        let mut i = 0;
        let mut add = |n: u8| {
            mem[i] = n;
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
        add(Register::R1 as u8);

        add(MOV_LIT_REG);
        add(0x56);
        add(0x78);
        add(Register::R4 as u8);

        add(PSH_LIT);
        add(0x00);
        add(0x00);

        add(CAL_LIT);
        add(subroutine_address.0);
        add(subroutine_address.1);

        add(PSH_LIT);
        add(0x44);
        add(0x44);

        add(HLT);

        // Subroutine..
        let mut i = 0x3000;
        let mut add = |n: u8| {
            mem[i] = n;
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
        add(Register::R1 as u8);

        add(MOV_LIT_REG);
        add(0x09);
        add(0x0A);
        add(Register::R8 as u8);

        add(RET);

        let mem = Memory::from_vec(mem);
        let mut cpu = CPU::new(mem);

        cpu.run();

        // Check that the state is the same as when we left it before calling the subroutine
        assert_eq!(cpu.pop(), 0x4444);
        assert_eq!(cpu.pop(), 0x1111);
        assert_eq!(cpu.pop(), 0x2222);
        assert_eq!(cpu.pop(), 0x3333);

        assert_eq!(cpu.get_register(Register::R1), 0x1234);
        assert_eq!(cpu.get_register(Register::R4), 0x5678);
    }
}