use crate::ram::Ram;

pub struct Cpu {
    sp : u8,  // Stack Pointer
    pc : u16, // Program Counter

    a : u8,   // Accumulator
    x : u8,   // Index Register X
    y : u8,   // Index Register Y

    carry_flag : bool,
    zero_flag : bool,
    interrupt_disable : bool,
    decimal_mode : bool,
    break_command : bool,
    overflow_flag : bool,
    negative_flag : bool,

    ram : Ram,

}

impl Cpu {
    pub fn create(ram: Ram) -> Cpu {
        Cpu {
            sp: (0), pc: (0),
            a: (0), x: (0), y: (0),
            carry_flag: (false), zero_flag: (false), interrupt_disable: (false),
            decimal_mode: (false), break_command: (false), overflow_flag: (false),
            negative_flag: (false),
            ram: (ram),
        }
    }

    pub fn read_instruction(&mut self) {
        let opcode = self.ram.read(self.pc.into());
        println!("opcode: {:#04x}", opcode);
        self.get_instruction(opcode)(self);
    }

    fn get_instruction(&mut self, opcode: u8) -> fn(&mut Cpu) {
        match opcode {
            // LDA
            0xA9 => Self::lda_imm,
            0xA5 => Self::lda_zp,
            0xB5 => Self::lda_zpx,
            0xAD => Self::lda_abs,
            0xBD => Self::lda_abx,
            0xB9 => Self::lda_aby,
            0xA1 => Self::lda_inx,
            0xB1 => Self::lda_iny,

            // LDX
            0xA2 => Self::ldx_imm,
            0xA6 => Self::ldx_zp,
            0xB6 => Self::ldx_zpy,
            0xAE => Self::ldx_abs,
            0xBE => Self::ldx_aby,

            // LDY
            0xA0 => Self::ldy_imm,
            0xA4 => Self::ldy_zp,
            0xB4 => Self::ldy_zpx,
            0xAC => Self::ldy_abs,
            0xBC => Self::ldy_abx,

            _ => unimplemented!("{} opcode not implemented yet!\n", opcode),
        }
    }

    fn imm(&mut self) -> u16 {
        self.pc += 1;
        self.pc
    }

    fn zp(&mut self) -> u16 {
        self.pc += 1;
        self.ram.read(self.pc) as u16
    }

    fn zpx(&mut self) -> u16 {
        self.pc += 1;
        (self.ram.read(self.pc) + self.x) as u16
    }

    fn zpy(&mut self) -> u16 {
        self.pc += 1;
        (self.ram.read(self.pc) + self.y) as u16
    }

    fn abs(&mut self) -> u16 {
        self.pc += 1;
        let addr = self.ram.read(self.pc);
        self.pc += 1;
        (self.ram.read(self.pc) as u16) << 8 | addr as u16
    }

    fn abx(&mut self) -> u16 {
        self.pc += 1;
        let addr = self.ram.read(self.pc);
        self.pc += 1;
        let addr = (self.ram.read(self.pc) as u16) << 8 | addr as u16;
        addr + self.x as u16
    }

    fn aby(&mut self) -> u16 {
        self.pc += 1;
        let addr = self.ram.read(self.pc);
        self.pc += 1;
        let addr = (self.ram.read(self.pc) as u16) << 8 | addr as u16;
        addr + self.y as u16
    }

    fn inx(&mut self) -> u16 {
        self.pc += 1;
        let addr: u16 = self.ram.read(self.pc) as u16;
        let addr = addr + self.x as u16;
        (self.ram.read(addr + 1) as u16) << 8 | self.ram.read(addr.into()) as u16
    }

    fn iny(&mut self) -> u16 {
        self.pc += 1;
        let addr: u16 = self.ram.read(self.pc) as u16;
        let addr = (self.ram.read(addr + 1) as u16) << 8 | self.ram.read(addr.into()) as u16;
        addr + self.y as u16
    }

    fn lda_imm(&mut self) {
        let addr = self.imm();
        self.a = self.ram.read(addr);

        self.zero_flag = self.a == 0;
        self.negative_flag = (self.a >> 7) == 1;

        self.pc += 1;
    }

    fn lda_zp(&mut self) {
        let addr = self.zp();
        self.a = self.ram.read(addr);

        self.zero_flag = self.a == 0;
        self.negative_flag = (self.a >> 7) == 1;

        self.pc += 1;
    }

    fn lda_zpx(&mut self) {
        let addr = self.zpx();
        self.a = self.ram.read(addr);

        self.zero_flag = self.a == 0;
        self.negative_flag = (self.a >> 7) == 1;

        self.pc += 1;
    }

    fn lda_abs(&mut self) {
        let addr = self.abs();
        self.a = self.ram.read(addr);

        self.zero_flag = self.a == 0;
        self.negative_flag = (self.a >> 7) == 1;

        self.pc += 1;
    }

    fn lda_abx(&mut self) {
        let addr = self.abx();
        self.a = self.ram.read(addr);

        self.zero_flag = self.a == 0;
        self.negative_flag = (self.a >> 7) == 1;

        self.pc += 1;
    }

    fn lda_aby(&mut self) {
        let addr = self.aby();
        self.a = self.ram.read(addr);

        self.zero_flag = self.a == 0;
        self.negative_flag = (self.a >> 7) == 1;

        self.pc += 1;
    }

    fn lda_inx(&mut self) {
        let addr = self.inx();
        self.a = self.ram.read(addr);

        self.zero_flag = self.a == 0;
        self.negative_flag = (self.a >> 7) == 1;

        self.pc += 1;
    }

    fn lda_iny(&mut self) {
        let addr = self.iny();
        self.a = self.ram.read(addr);

        self.zero_flag = self.a == 0;
        self.negative_flag = (self.a >> 7) == 1;

        self.pc += 1;
    }

    fn ldx_imm(&mut self) {
        let addr = self.imm();
        self.x = self.ram.read(addr);

        self.zero_flag = self.x == 0;
        self.negative_flag = (self.x >>  7) == 1;

        self.pc += 1;
    }

    fn ldx_zp(&mut self) {
        let addr = self.zp();
        self.x = self.ram.read(addr);

        self.zero_flag = self.x == 0;
        self.negative_flag = (self.x >>  7) == 1;

        self.pc += 1;
    }

    fn ldx_zpy(&mut self) {
        let addr = self.zpy();
        self.x = self.ram.read(addr);

        self.zero_flag = self.x == 0;
        self.negative_flag = (self.x >>  7) == 1;

        self.pc += 1;
    }

    fn ldx_abs(&mut self) {
        let addr = self.abs();
        self.x = self.ram.read(addr);

        self.zero_flag = self.x == 0;
        self.negative_flag = (self.x >>  7) == 1;

        self.pc += 1;
    }

    fn ldx_aby(&mut self) {
        let addr = self.aby();
        self.x = self.ram.read(addr);

        self.zero_flag = self.x == 0;
        self.negative_flag = (self.x >>  7) == 1;

        self.pc += 1;
    }

    fn ldy_imm(&mut self) {
        let addr = self.imm();
        self.y = self.ram.read(addr);

        self.zero_flag = self.y == 0;
        self.negative_flag = (self.y >>  7) == 1;

        self.pc += 1;
    }

    fn ldy_zp(&mut self) {
        let addr = self.zp();
        self.y = self.ram.read(addr);

        self.zero_flag = self.y == 0;
        self.negative_flag = (self.y >>  7) == 1;

        self.pc += 1;
    }

    fn ldy_zpx(&mut self) {
        let addr = self.zpx();
        self.y = self.ram.read(addr);

        self.zero_flag = self.y == 0;
        self.negative_flag = (self.y >>  7) == 1;

        self.pc += 1;
    }

    fn ldy_abs(&mut self) {
        let addr = self.abs();
        self.y = self.ram.read(addr);

        self.zero_flag = self.y == 0;
        self.negative_flag = (self.y >>  7) == 1;

        self.pc += 1;
    }

    fn ldy_abx(&mut self) {
        let addr = self.abx();
        self.y = self.ram.read(addr);

        self.zero_flag = self.y == 0;
        self.negative_flag = (self.y >>  7) == 1;

        self.pc += 1;
    }

}

#[test]
fn lda_imm() {
    let mut ram = Ram::create();
    ram.write(0x0, 0xA9);
    ram.write(0x1, 0x42);
    let mut cpu = Cpu::create(ram);
    cpu.read_instruction();

    assert_eq!(cpu.a, 0x42);
    assert_eq!(cpu.pc, 0x2);
}

#[test]
fn lda_zp() {
    let mut ram = Ram::create();
    ram.write(0x0, 0xA5);
    ram.write(0x1, 0x69);
    ram.write(0x69, 0x42);
    let mut cpu = Cpu::create(ram);
    cpu.read_instruction();

    assert_eq!(cpu.a, 0x42);
    assert_eq!(cpu.pc, 0x2);
}

#[test]
fn lda_zpx() {
    let mut ram = Ram::create();
    ram.write(0x0, 0xB5);
    ram.write(0x1, 0x69);
    ram.write(0x69 + 0x5, 0x42);
    let mut cpu = Cpu::create(ram);
    cpu.x = 0x5;
    cpu.read_instruction();

    assert_eq!(cpu.a, 0x42);
    assert_eq!(cpu.pc, 0x2);
}

#[test]
fn lda_abs() {
    let mut ram = Ram::create();
    ram.write(0x0, 0xAD);
    ram.write(0x1, 0x32);
    ram.write(0x2, 0x44);
    ram.write(0x4432, 0x42);
    let mut cpu = Cpu::create(ram);
    cpu.read_instruction();

    assert_eq!(cpu.a, 0x42);
    assert_eq!(cpu.pc, 0x3);
}

#[test]
fn lda_abx() {
    let mut ram = Ram::create();
    ram.write(0x0, 0xBD);
    ram.write(0x1, 0x32);
    ram.write(0x2, 0x44);
    ram.write(0x4432 + 0x3, 0x42);
    let mut cpu = Cpu::create(ram);
    cpu.x = 0x3;
    cpu.read_instruction();

    assert_eq!(cpu.a, 0x42);
    assert_eq!(cpu.pc, 0x3);
}

#[test]
fn lda_aby() {
    let mut ram = Ram::create();
    ram.write(0x0, 0xB9);
    ram.write(0x1, 0x32);
    ram.write(0x2, 0x44);
    ram.write(0x4432 + 0x3, 0x42);
    let mut cpu = Cpu::create(ram);
    cpu.y = 0x3;
    cpu.read_instruction();

    assert_eq!(cpu.a, 0x42);
    assert_eq!(cpu.pc, 0x3);
}

#[test]
fn lda_inx() {
    let mut ram = Ram::create();
    ram.write(0x0, 0xa1);
    ram.write(0x1, 0x42);
    ram.write(0x42 + 0x02, 0x69);
    ram.write(0x43 + 0x02, 0x35);
    ram.write(0x3569, 0x55);
    let mut cpu = Cpu::create(ram);
    cpu.x = 0x2;
    cpu.read_instruction();

    assert_eq!(cpu.a, 0x55);
    assert_eq!(cpu.pc, 0x2);
}

#[test]
fn lda_iny() {
    let mut ram = Ram::create();
    ram.write(0x0, 0xB1);
    ram.write(0x1, 0x42);
    ram.write(0x42, 0x69);
    ram.write(0x43, 0x35);
    ram.write(0x3569 + 0x2, 0x55);
    let mut cpu = Cpu::create(ram);
    cpu.y = 0x2;
    cpu.read_instruction();

    assert_eq!(cpu.a, 0x55);
    assert_eq!(cpu.pc, 0x2);
}

#[test]
fn lda_zero_flag() {
    let mut ram = Ram::create();
    ram.write(0x0, 0xA9);
    ram.write(0x1, 0x00);
    ram.write(0x2, 0xA9);
    ram.write(0x3, 0x01);
    let mut cpu = Cpu::create(ram);

    cpu.read_instruction();
    assert!(cpu.zero_flag);

    cpu.read_instruction();
    assert!(!cpu.zero_flag);
}

#[test]
fn lda_negative_flag() {
    let mut ram = Ram::create();
    ram.write(0x0, 0xA9);
    ram.write(0x1, 0x80);
    ram.write(0x2, 0xA9);
    ram.write(0x3, 0x01);
    let mut cpu = Cpu::create(ram);

    cpu.read_instruction();
    assert!(cpu.negative_flag);

    cpu.read_instruction();
    assert!(!cpu.negative_flag);
}

#[test]
fn ldx_imm() {
    let mut ram = Ram::create();
    ram.write(0x0, 0xA2);
    ram.write(0x1, 0x42);
    let mut cpu = Cpu::create(ram);
    cpu.read_instruction();

    assert_eq!(cpu.x, 0x42);
    assert_eq!(cpu.pc, 0x2);
}

#[test]
fn ldx_zp() {
    let mut ram = Ram::create();
    ram.write(0x0, 0xA6);
    ram.write(0x1, 0x69);
    ram.write(0x69, 0x42);
    let mut cpu = Cpu::create(ram);
    cpu.read_instruction();

    assert_eq!(cpu.x, 0x42);
    assert_eq!(cpu.pc, 0x2);
}

#[test]
fn ldx_zpy() {
    let mut ram = Ram::create();
    ram.write(0x0, 0xB6);
    ram.write(0x1, 0x69);
    ram.write(0x69 + 0x5, 0x42);
    let mut cpu = Cpu::create(ram);
    cpu.y = 0x5;
    cpu.read_instruction();

    assert_eq!(cpu.x, 0x42);
    assert_eq!(cpu.pc, 0x2);
}

#[test]
fn ldx_abs() {
    let mut ram = Ram::create();
    ram.write(0x0, 0xAE);
    ram.write(0x1, 0x32);
    ram.write(0x2, 0x44);
    ram.write(0x4432, 0x42);
    let mut cpu = Cpu::create(ram);
    cpu.read_instruction();

    assert_eq!(cpu.x, 0x42);
    assert_eq!(cpu.pc, 0x3);
}

#[test]
fn ldx_aby() {
    let mut ram = Ram::create();
    ram.write(0x0, 0xBE);
    ram.write(0x1, 0x32);
    ram.write(0x2, 0x44);
    ram.write(0x4432 + 0x3, 0x42);
    let mut cpu = Cpu::create(ram);
    cpu.y = 0x3;
    cpu.read_instruction();

    assert_eq!(cpu.x, 0x42);
    assert_eq!(cpu.pc, 0x3);
}

#[test]
fn ldx_zero_flag() {
    let mut ram = Ram::create();
    ram.write(0x0, 0xA2);
    ram.write(0x1, 0x00);
    ram.write(0x2, 0xA2);
    ram.write(0x3, 0x01);
    let mut cpu = Cpu::create(ram);

    cpu.read_instruction();
    assert!(cpu.zero_flag);

    cpu.read_instruction();
    assert!(!cpu.zero_flag);
}

#[test]
fn ldx_negative_flag() {
    let mut ram = Ram::create();
    ram.write(0x0, 0xA2);
    ram.write(0x1, 0x80);
    ram.write(0x2, 0xA2);
    ram.write(0x3, 0x01);
    let mut cpu = Cpu::create(ram);

    cpu.read_instruction();
    assert!(cpu.negative_flag);

    cpu.read_instruction();
    assert!(!cpu.negative_flag);
}

#[test]
fn ldy_imm() {
    let mut ram = Ram::create();
    ram.write(0x0, 0xA0);
    ram.write(0x1, 0x42);
    let mut cpu = Cpu::create(ram);
    cpu.read_instruction();

    assert_eq!(cpu.y, 0x42);
    assert_eq!(cpu.pc, 0x2);
}

#[test]
fn ldy_zp() {
    let mut ram = Ram::create();
    ram.write(0x0, 0xA4);
    ram.write(0x1, 0x69);
    ram.write(0x69, 0x42);
    let mut cpu = Cpu::create(ram);
    cpu.read_instruction();

    assert_eq!(cpu.y, 0x42);
    assert_eq!(cpu.pc, 0x2);
}

#[test]
fn ldy_zpx() {
    let mut ram = Ram::create();
    ram.write(0x0, 0xB4);
    ram.write(0x1, 0x69);
    ram.write(0x69 + 0x5, 0x42);
    let mut cpu = Cpu::create(ram);
    cpu.x = 0x5;
    cpu.read_instruction();

    assert_eq!(cpu.y, 0x42);
    assert_eq!(cpu.pc, 0x2);
}

#[test]
fn ldy_abs() {
    let mut ram = Ram::create();
    ram.write(0x0, 0xAC);
    ram.write(0x1, 0x32);
    ram.write(0x2, 0x44);
    ram.write(0x4432, 0x42);
    let mut cpu = Cpu::create(ram);
    cpu.read_instruction();

    assert_eq!(cpu.y, 0x42);
    assert_eq!(cpu.pc, 0x3);
}

#[test]
fn ldy_abx() {
    let mut ram = Ram::create();
    ram.write(0x0, 0xBC);
    ram.write(0x1, 0x32);
    ram.write(0x2, 0x44);
    ram.write(0x4432 + 0x3, 0x42);
    let mut cpu = Cpu::create(ram);
    cpu.x = 0x3;
    cpu.read_instruction();

    assert_eq!(cpu.y, 0x42);
    assert_eq!(cpu.pc, 0x3);
}

#[test]
fn ldy_zero_flag() {
    let mut ram = Ram::create();
    ram.write(0x0, 0xA0);
    ram.write(0x1, 0x00);
    ram.write(0x2, 0xA0);
    ram.write(0x3, 0x01);
    let mut cpu = Cpu::create(ram);

    cpu.read_instruction();
    assert!(cpu.zero_flag);

    cpu.read_instruction();
    assert!(!cpu.zero_flag);
}

#[test]
fn ldy_negative_flag() {
    let mut ram = Ram::create();
    ram.write(0x0, 0xA0);
    ram.write(0x1, 0x80);
    ram.write(0x2, 0xA0);
    ram.write(0x3, 0x01);
    let mut cpu = Cpu::create(ram);

    cpu.read_instruction();
    assert!(cpu.negative_flag);

    cpu.read_instruction();
    assert!(!cpu.negative_flag);
}
