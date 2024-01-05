use std::any::Any;
use std::env;
use cursive::Cursive;
use cursive::views::{Dialog, SelectView};
use cursive_hexview::{DisplayState, HexView};
use cursive::traits::*;
use mayo_lib::cpu::CPU;
use mayo_lib::cpu::instructions::*;
use mayo_lib::cpu::register::Register;
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
            tui_mode();
        }
    } else {
        // no cli args were passed to the program
        tui_mode();
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


    let memory = Box::new(memory);

    let mut mm = MemoryMapper::new();
    mm.map(memory, 0, 0xFFFF, true);

    let screen_device = Box::new(ScreenDevice::new());
    mm.map(screen_device, 0x3000, 0x30FF, true);

    let mut cpu = CPU::new(Box::new(mm));

    cpu.run();
}

fn tui_mode() {
    let mut siv = cursive::default();

    #[derive(Copy, Clone)]
    enum MemoryType {
        Basic,
        Mapped
    }

    let mem_type_select = SelectView::<MemoryType>::new()
        .item("Basic", MemoryType::Basic)
        .item("Mapped", MemoryType::Mapped)
        .on_submit(|s, mem_type| {
            match mem_type {
                MemoryType::Basic => {
                    s.pop_layer();
                    edit_basic_memory(s);
                }
                MemoryType::Mapped => {
                    s.add_layer(Dialog::info("Mapped is not yet implemented"));
                }
            }
        });

    siv.add_layer(Dialog::around(mem_type_select).title("Select a type of memory"));

    siv.run();
}

fn edit_basic_memory(s: &mut Cursive) {
    let mem_view = HexView::new()
        .display_state(DisplayState::Editable)
        .with_name("mem_view");

    s.add_layer(Dialog::around(mem_view)
        .title("Edit the memory")
        .button("Run", |s| {
            // run the cpu
            let mut mem_view = s.find_name::<HexView>("mem_view").unwrap();

            let data = mem_view.data().to_owned();

            let mut cpu = CPU::new(Box::new(data));
            cpu.run();

            let final_mem: Box<Vec<u8>> = get(cpu.memory.into_any());

            mem_view.set_data(final_mem.into_iter());
        }));
}

fn get<T: Any>(value: Box<dyn Any>) -> Box<T> {
    let pv = value.downcast().expect("The pointed-to value must be of type T");
    pv
}