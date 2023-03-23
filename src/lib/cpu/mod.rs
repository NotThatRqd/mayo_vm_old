use std::collections::HashMap;
use data_view::View;
use enum_iterator::{all, cardinality};
use crate::cpu::instructions::*;
use crate::cpu::register::Register;
use crate::create_memory::create_memory;
use crate::devices::device::Device;

pub mod instructions;
pub mod register;
pub mod memory;

#[derive(Debug)]
pub enum ExecuteError {
    UnknownInstruction(u8),
    NullByte,
}

pub struct CPU<T>
where
    T: Device
{
    memory: T,

    registers: Vec<u8>,
    register_map: HashMap<Register, usize>,

    stack_frame_size: u16,
}

impl<T> CPU<T>
where
    T: Device
{
    pub fn new(memory: T) -> Self {
        let mut register_map = HashMap::new();

        for (i, register) in all::<Register>().enumerate() {
            register_map.insert(register, i * 2);
        }

        let mut cpu = CPU {
            memory,
            // multiplied by two because each register is two bytes big
            registers: create_memory(cardinality::<Register>() * 2),
            register_map,
            stack_frame_size: 0,
        };

        cpu.set_register(Register::Sp, 0xFFFF - 1);
        cpu.set_register(Register::Fp, 0xFFFF - 1);

        cpu
    }

    pub fn debug(&self) {
        for register in all::<Register>() {
            println!("{:?}: 0x{:04X?}", register, self.get_register(register));
        }
    }

    pub fn view_memory_at(&self, address: usize, n: usize) -> Result<(), ()> {
        let mut next_n_bytes = vec![];
        for i in 0..=n {
            let next = self.memory.read_at_u8(address + i);
            if let Some(next) = next {
                next_n_bytes.push(next);
            } else {
                return Err(());
            }
        }

        println!("0x{:04X?}: {:02X?}", address, next_n_bytes);

        Ok(())
    }

    pub fn get_register(&self, register: Register) -> u16 {
        let index = self.register_map.get(&register)
            .expect(&format!("register {:?} not in self.register_map", register));

        let index = *index;

        self.registers.read_at::<u16>(index)
            .expect("read register")
    }

    fn set_register(&mut self, register: Register, value: u16) {
        let index = self.register_map.get(&register)
            .expect(&format!("register {:?} not in self.register_map", register));

        let index = *index;

        // We panic instead of returning an Err() because we know that the index is correct, unless the cpu_dict test is wrong
        self.registers.write_at::<u16>(index, value)
            .expect("write to register");
    }

    fn fetch(&mut self) -> u8 {
        let next_instruction_address = self.get_register(Register::Ip);
        let instruction = self.memory.read_at_u8(next_instruction_address as usize)
            .unwrap();
        self.set_register(Register::Ip, next_instruction_address + 1);

        instruction
    }

    fn fetch16(&mut self) -> u16 {
        let next_instruction_address = self.get_register(Register::Ip);
        let instruction = self.memory.read_at_u16(next_instruction_address as usize)
            .unwrap();
        self.set_register(Register::Ip, next_instruction_address + 2);

        instruction
    }

    fn fetch_register_index(&mut self) -> usize {
        // multiplied by two because each register is two bytes long
        (self.fetch() as usize % self.register_map.len()) * 2
    }

    fn push(&mut self, value: u16) {
        let sp_address = self.get_register(Register::Sp);
        self.memory.write_at_u16(sp_address as usize, value).unwrap();
        self.set_register(Register::Sp, sp_address - 2);
        self.stack_frame_size += 2;
    }

    fn pop(&mut self) -> u16 {
        let next_sp_address = self.get_register(Register::Sp) + 2;
        self.set_register(Register::Sp, next_sp_address);
        self.stack_frame_size -= 2;
        self.memory.read_at_u16(next_sp_address as usize).unwrap()
    }

    fn push_state(&mut self) {
        self.push(self.get_register(Register::R1));
        self.push(self.get_register(Register::R2));
        self.push(self.get_register(Register::R3));
        self.push(self.get_register(Register::R4));
        self.push(self.get_register(Register::R5));
        self.push(self.get_register(Register::R6));
        self.push(self.get_register(Register::R7));
        self.push(self.get_register(Register::R8));
        self.push(self.get_register(Register::Ip));
        self.push(self.stack_frame_size + 2);

        self.set_register(Register::Fp, self.get_register(Register::Sp));
        self.stack_frame_size = 0;
    }

    fn pop_state(&mut self) {
        let frame_pointer_address = self.get_register(Register::Fp);
        self.set_register(Register::Sp, frame_pointer_address);

        self.stack_frame_size = self.pop();
        let stack_frame_size = self.stack_frame_size;

        let mut pop_into_reg = |register: Register| {
            let value = self.pop();
            self.set_register(register, value);
        };

        pop_into_reg(Register::Ip);
        pop_into_reg(Register::R8);
        pop_into_reg(Register::R7);
        pop_into_reg(Register::R6);
        pop_into_reg(Register::R5);
        pop_into_reg(Register::R4);
        pop_into_reg(Register::R3);
        pop_into_reg(Register::R2);
        pop_into_reg(Register::R1);

        let n_args = self.pop();
        for _i in 0..n_args {
            self.pop();
        }

        self.set_register(Register::Fp, frame_pointer_address + stack_frame_size);
    }

    fn execute(&mut self, instruction: u8) -> Result<bool, ExecuteError> {
        match instruction {
            MOV_LIT_REG => {
                let literal = self.fetch16();
                let register = self.fetch_register_index();
                self.registers.write_at::<u16>(register, literal)
                    .unwrap();
            }

            MOV_REG_REG => {
                let reg_from = self.fetch_register_index();
                let reg_to = self.fetch_register_index();
                let value = self.registers.read_at::<u16>(reg_from)
                    .unwrap();
                self.registers.write_at::<u16>(reg_to, value)
                    .unwrap();
            }

            MOV_REG_MEM => {
                let reg_from = self.fetch_register_index();
                let address = self.fetch16() as usize;
                let value = self.registers.read_at::<u16>(reg_from)
                    .unwrap();
                self.memory.write_at_u16(address, value)
                    .unwrap();
            }

            MOV_MEM_REG => {
                let address = self.fetch16() as usize;
                let reg_to = self.fetch_register_index();
                let value = self.memory.read_at_u16(address)
                    .unwrap();
                self.registers.write_at::<u16>(reg_to, value)
                    .unwrap();
            }

            ADD_REG_REG => {
                let reg1 = self.fetch_register_index();
                let reg2 = self.fetch_register_index();
                let reg1_value = self.registers.read_at::<u16>(reg1).unwrap();
                let reg2_value = self.registers.read_at::<u16>(reg2).unwrap();

                self.set_register(Register::Acc, reg1_value + reg2_value);
            }

            JMP_NOT_EQ => {
                let value = self.fetch16();
                let address = self.fetch16();

                if value != self.get_register(Register::Acc) {
                    self.set_register(Register::Ip, address);
                }
            }

            PSH_LIT => {
                let value = self.fetch16();
                self.push(value);
            }

            PSH_REG => {
                let reg = self.fetch_register_index();
                let value = self.registers.read_at::<u16>(reg).unwrap();
                self.push(value);
            }

            POP => {
                let reg = self.fetch_register_index();
                let value = self.pop();
                self.registers.write_at::<u16>(reg, value).unwrap();
            }

            CAL_LIT => {
                let address = self.fetch16();
                self.push_state();

                self.set_register(Register::Ip, address);
            }

            CAL_REG => {
                let reg = self.fetch_register_index();
                let address = self.registers.read_at::<u16>(reg).unwrap();
                self.push_state();

                self.set_register(Register::Ip, address);
            }

            RET => {
                self.pop_state();
            }

            HLT => {
                return Ok(true);
            }

            0x00 => {
                return Err(ExecuteError::NullByte);
            }

            _ => {
                return Err(ExecuteError::UnknownInstruction(instruction));
            }
        };
        Ok(false)
    }

    pub fn step(&mut self) -> Result<bool, ExecuteError> {
        let instruction = self.fetch();
        self.execute(instruction)
    }

    pub fn run(&mut self) {
        loop {
            let should_halt = self.step()
                .unwrap();
            if should_halt {
                break;
            }
        }
    }
}

// TODO: fix some warnings generated by the tests
#[cfg(test)]
mod tests {
    use crate::cpu::{CPU, ExecuteError, Register};
    use crate::cpu::instructions::*;
    use crate::cpu::memory::Memory;
    use crate::create_memory::create_memory;

    #[test]
    fn cpu_dict() {
        let mem = Memory::from_num_of_bytes(8);
        let cpu = CPU::new(mem);

        assert_eq!(cpu.register_map.get(&Register::Ip), Some(&0));
        assert_eq!(cpu.register_map.get(&Register::Acc), Some(&2));
        assert_eq!(cpu.register_map.get(&Register::R1), Some(&4));
        assert_eq!(cpu.register_map.get(&Register::R2), Some(&6));
        assert_eq!(cpu.register_map.get(&Register::R3), Some(&8));
        assert_eq!(cpu.register_map.get(&Register::R4), Some(&10));
        assert_eq!(cpu.register_map.get(&Register::R5), Some(&12));
        assert_eq!(cpu.register_map.get(&Register::R6), Some(&14));
        assert_eq!(cpu.register_map.get(&Register::R7), Some(&16));
        assert_eq!(cpu.register_map.get(&Register::R8), Some(&18));
        assert_eq!(cpu.register_map.get(&Register::Sp), Some(&20));
        assert_eq!(cpu.register_map.get(&Register::Fp), Some(&22));
    }

    const R1: u8  = 2;
    const R2: u8  = 3;
    const R8: u8  = 9;
    const R4: u8  = 5;

    #[test]
    fn subroutine() {
        let mut memory = create_memory(256 * 256);

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


        let memory = Memory::from_vec(memory);
        let mut cpu = CPU::new(memory);

        loop {
            if let Err(e) = cpu.step() {
                match e {
                    ExecuteError::UnknownInstruction(instruction) => panic!("unknown instruction {:02X?}", instruction),
                    ExecuteError::NullByte => break,
                }
            }
        }

        // Check that the state is the same as when we left it before calling the subroutine
        assert_eq!(cpu.pop(), 0x4444);
        assert_eq!(cpu.pop(), 0x1111);
        assert_eq!(cpu.pop(), 0x2222);
        assert_eq!(cpu.pop(), 0x3333);

        assert_eq!(cpu.get_register(Register::R1), 0x1234);
        assert_eq!(cpu.get_register(Register::R4), 0x5678);
    }

    #[test]
    fn addition_program() {
        let mut memory = create_memory(16);

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


        let memory = Memory::from_vec(memory);
        let mut cpu = CPU::new(memory);
        cpu.step();
        cpu.step();
        cpu.step();

        let acc_value = cpu.get_register(Register::Acc);
        assert_eq!(acc_value, 0x1234 + 0xABCD);
    }
}
