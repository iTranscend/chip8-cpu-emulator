struct CPU {
    registers: [u8; 16],
    position_in_memory: usize,
    memory: [u8; 0x1000],
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        // combining two bytes into a u16
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        op_byte1 << 8 | op_byte2
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.position_in_memory += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = (opcode & 0x000F) as u8;

            match (c, x, y, d) {
                (0x0, 0x0, 0xE, 0x0) => self.clear_screen(),
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                (0x0, 0x0, 0x0, 0x0) => break,
                _ => todo!("Undefined opcode: {:x}", opcode),
            }
        }

        println!("CPU halted..... Exiting gracefully");
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (value, is_overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = value;

        if is_overflow {
            self.registers[0x000F] = 1;
        } else {
            self.registers[0x000F] = 0;
        }
    }

    fn clear_screen(&mut self) {
        todo!("clear_screen");
    }
}

fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        position_in_memory: 0,
        memory: [0x0000; 4096],
    };

    cpu.registers[0] = 2;
    cpu.registers[1] = 10;
    cpu.registers[2] = 20;
    cpu.registers[3] = 30;

    let mem = &mut cpu.memory;
    mem[0] = 0x80;
    mem[1] = 0x14;
    mem[2] = 0x80;
    mem[3] = 0x24;
    mem[4] = 0x80;
    mem[5] = 0x34;

    cpu.run();
    println!("Sum of all: {}", cpu.registers[0]);
}
