use std::collections::HashMap;
use data_view::View;
use enum_iterator::{all, cardinality};
use crate::cpu::instructions::*;
use crate::cpu::register::Register;
use crate::create_memory::create_memory;

pub mod instructions;
pub mod register;

pub struct CPU {
    memory: Vec<u8>,

    registers: Vec<u8>,
    register_map: HashMap<Register, usize>,
}

impl CPU {
    pub fn new(memory: Vec<u8>) -> CPU {
        let mut register_map = HashMap::new();

        for (i, register) in all::<Register>().enumerate() {
            register_map.insert(register, i * 2);
        }

        let mut cpu = CPU {
            memory,
            // multiplied by two because each register is two bytes big
            registers: create_memory(cardinality::<Register>() * 2),
            register_map
        };

        cpu.set_register(Register::Sp, (cpu.memory.len() - 1 - 1) as u16)
            .unwrap();
        cpu.set_register(Register::Fp, (cpu.memory.len() - 1 - 1) as u16)
            .unwrap();

        cpu
    }

    pub fn debug(&self) {
        for register in all::<Register>() {
            println!("{:?}: 0x{:04X?}", register, self.get_register(register).unwrap());
        }
    }

    pub fn view_memory_at(&self, address: usize) -> Result<(), ()> {
        let mut next_eight_bytes = vec![];
        for i in 0..=8 {
            let next = self.memory.read_at::<u8>(address + i);
            if let Some(n) = next {
                next_eight_bytes.push(n);
            } else {
                return Err(());
            }
        }

        println!("0x{:04X?}: {:02X?}", address, next_eight_bytes);

        Ok(())
    }

    pub fn get_register(&self, register: Register) -> Option<u16> {
        let index = self.register_map.get(&register)
            .expect(&format!("register {:?} not in self.register_map", register));

        let index = *index;

        self.registers.read_at::<u16>(index)
    }

    fn set_register(&mut self, register: Register, value: u16) -> Result<(), &str> {
        let index = self.register_map.get(&register)
            .expect(&format!("register {:?} not in self.register_map", register));

        let index = *index;

        match self.registers.write_at::<u16>(index, value) {
            Ok(_) => Ok(()),
            Err(_) => Err("error writing to register")
        }
    }

    fn fetch(&mut self) -> u8 {
        let next_instruction_address = self.get_register(Register::Ip)
            .unwrap();
        let instruction = self.memory.read_at::<u8>(next_instruction_address as usize)
            .unwrap();
        self.set_register(Register::Ip, next_instruction_address + 1)
            .unwrap();

        instruction
    }

    fn fetch16(&mut self) -> u16 {
        let next_instruction_address = self.get_register(Register::Ip)
            .unwrap();
        let instruction = self.memory.read_at::<u16>(next_instruction_address as usize)
            .unwrap();
        self.set_register(Register::Ip, next_instruction_address + 2)
            .unwrap();

        instruction
    }

    fn fetch_register_index(&mut self) -> usize {
        // multiplied by two because each register is two bytes long
        (self.fetch() as usize % self.register_map.len()) * 2
    }

    fn push(&mut self, value: u16) {
        let sp_address = self.get_register(Register::Sp).unwrap();
        self.memory.write_at::<u16>(sp_address as usize, value).unwrap();
        self.set_register(Register::Sp, sp_address - 2).unwrap();
    }

    fn pop(&mut self) -> u16 {
        let next_sp_address = self.get_register(Register::Sp).unwrap() + 2;
        self.set_register(Register::Sp, next_sp_address).unwrap();
        self.memory.read_at::<u16>(next_sp_address as usize).unwrap()
    }

    fn execute(&mut self, instruction: u8) -> bool {
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
                self.memory.write_at::<u16>(address, value)
                    .unwrap();
            }

            MOV_MEM_REG => {
                let address = self.fetch16() as usize;
                let reg_to = self.fetch_register_index();
                let value = self.memory.read_at::<u16>(address)
                    .unwrap();
                self.registers.write_at::<u16>(reg_to, value)
                    .unwrap();
            }

            ADD_REG_REG => {
                let reg1 = self.fetch_register_index();
                let reg2 = self.fetch_register_index();
                let reg1_value = self.registers.read_at::<u16>(reg1).unwrap();
                let reg2_value = self.registers.read_at::<u16>(reg2).unwrap();

                self.set_register(Register::Acc, reg1_value + reg2_value).unwrap();
            }

            JMP_NOT_EQ => {
                let value = self.fetch16();
                let address = self.fetch16();

                if value != self.get_register(Register::Acc).unwrap() {
                    self.set_register(Register::Ip, address)
                        .unwrap();
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

            _ => {
                panic!("unknown instruction 0x{:02X?}", instruction);
            }
        };
        false
    }

    pub fn step(&mut self) -> bool {
        let instruction = self.fetch();
        self.execute(instruction)
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::{CPU, Register};
    use crate::cpu::instructions::{ADD_REG_REG, MOV_LIT_REG};
    use crate::create_memory::create_memory;

    #[test]
    fn cpu_dict() {
        let mem = create_memory(1);
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


        let mut cpu = CPU::new(memory);
        cpu.step();
        cpu.step();
        cpu.step();

        let acc_value = cpu.get_register(Register::Acc).unwrap();
        assert_eq!(acc_value, 0x1234 + 0xABCD);
    }
}
