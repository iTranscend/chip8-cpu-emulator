struct CPU {
  current_operation: u16,
  registers: [u8; 2],
}

impl CPU {
  fn read_opcode(&self) -> u16 {
    self.current_operation
  }

  fn run(&mut self) {
    let opcode = self.read_opcode();
    let c = ((opcode & 0xF000) >> 12) as u8;
    let x = ((opcode & 0x0F00) >> 8) as u8;
    let y = ((opcode & 0x00F0) >> 4) as u8;
    let d = (opcode & 0x000F) as u8;

    match (c, x, y, d) {
      (0x0, 0x0, 0xE, 0x0) => self.clear_screen(),
      (0x8, _, _, 0x4) => self.add_xy(x, y),
      _ => todo!("Unset opcode: {:x}", opcode),
    }
  }

  fn add_xy(&mut self, x: u8, y: u8) {
    self.registers[x as usize] += self.registers[y as usize];
  }

  fn clear_screen(&mut self) {
    todo!("clear_screen");
  }
}

fn main() {
    let mut cpu = CPU {
      current_operation: 0x8014,
      registers: [5, 65],
    };

    cpu.run();
    println!("{}", cpu.registers[0]);
}
