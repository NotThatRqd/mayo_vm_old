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

        CPU {
            memory,
            // multiplied by two because each register is two bytes big
            registers: create_memory(cardinality::<Register>() * 2),
            register_map
        }
    }

    pub fn debug(&self) {
        for register in all::<Register>() {
            println!("{:?}: 0x{:02X?}", register, self.get_register(register).unwrap());
        }
    }

    pub fn view_memory_at(&self, address: usize) {
        let mut next_eight_bytes = vec![];
        for i in 0..=8 {
            let next = self.memory.read_at::<u8>(address + i)
                .unwrap();
            next_eight_bytes.push(next);
        }

        println!("0x{:04X?}: {:02X?}", address, next_eight_bytes);
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
                let reg1 = self.fetch() as usize;
                let reg2 = self.fetch() as usize;
                let reg1_value = self.registers.read_at::<u16>(reg1 * 2).unwrap();
                let reg2_value = self.registers.read_at::<u16>(reg2 * 2).unwrap();

                self.set_register(Register::Acc, reg1_value + reg2_value).unwrap();
            }

            _ => {
                panic!("unknown instruction {:02X?}", instruction);
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
    }
}
