use crate::ram::Ram;

pub struct Cpu {
    sp : u8,  // Stack Pointer
    pc : u16, // Program Counter

    a : u8,   // Accumulator
    x : u8,   // Index Register X
    y : u8,   // Index Register Y

    status: u8,

    ram : Ram,

}

macro_rules! instr {
    ($instruction:ident-imp) => {
        |cpu: &mut Cpu| {
            cpu.$instruction();
        }
    };
    ($instruction:ident-$addr_mode:ident) => {
        |cpu: &mut Cpu| {
            let addr = cpu.$addr_mode();
            cpu.$instruction(addr);
        }
    }
}

impl Cpu {
    pub fn create(ram: Ram) -> Cpu {
        Cpu {
            sp: 0xff, pc: 0,
            a: 0, x: 0, y: 0,
            status: 0,
            ram,
        }
    }

    fn get_carry_flag(&self) -> bool {
        self.status & 1 << 0 != 0
    }

    fn get_zero_flag(&self) -> bool {
        self.status & 1 << 1 != 0
    }

    fn set_zero_flag(&mut self){
        self.status |= 1 << 1
    }

    fn get_interrupt_flag(&self) -> bool {
        self.status & 1 << 2 != 0
    }

    fn get_decimal_mode(&self) -> bool {
        self.status & 1 << 3 != 0
    }

    fn get_break_command(&self) -> bool {
        self.status & 1 << 4 != 0
    }

    fn get_overflow_flag(&self) -> bool {
        self.status & 1 << 5 != 0
    }

    fn get_negative_flag(&self) -> bool {
        self.status & 1 << 6 != 0
    }

    fn set_negative_flag(&mut self) {
        self.status |= 1 << 6
    }

    pub fn read_instruction(&mut self) {
        let opcode = self.ram.read(self.pc.into());
        println!("opcode: {:#04x}", opcode);
        self.get_instruction(opcode)(self);
    }

    fn get_instruction(&mut self, opcode: u8) -> fn(&mut Cpu) {
        match opcode {
            // LDA
            0xA9 => instr!(lda-imm),
            0xA5 => instr!(lda-zp),
            0xB5 => instr!(lda-zpx),
            0xAD => instr!(lda-abs),
            0xBD => instr!(lda-abx),
            0xB9 => instr!(lda-aby),
            0xA1 => instr!(lda-inx),
            0xB1 => instr!(lda-iny),

            // LDX
            0xA2 => instr!(ldx-imm),
            0xA6 => instr!(ldx-zp),
            0xB6 => instr!(ldx-zpy),
            0xAE => instr!(ldx-abs),
            0xBE => instr!(ldx-aby),

            // LDY
            0xA0 => instr!(ldy-imm),
            0xA4 => instr!(ldy-zp),
            0xB4 => instr!(ldy-zpx),
            0xAC => instr!(ldy-abs),
            0xBC => instr!(ldy-abx),

            //STA
            0x85 => instr!(sta-zp),
            0x95 => instr!(sta-zpx),
            0x8D => instr!(sta-abs),
            0x9D => instr!(sta-abx),
            0x99 => instr!(sta-aby),
            0x81 => instr!(sta-inx),
            0x91 => instr!(sta-iny),

            //STX
            0x86 => instr!(stx-zp),
            0x96 => instr!(stx-zpy),
            0x8E => instr!(stx-abs),

            //STY
            0x84 => instr!(sty-zp),
            0x94 => instr!(sty-zpx),
            0x8C => instr!(sty-abs),

            //TAX
            0xAA => instr!(tax-imp),

            //TAY
            0xA8 => instr!(tay-imp),

            //TXA
            0x8A => instr!(txa-imp),

            //TYA
            0x98 => instr!(tya-imp),

            //TSX
            0xBA => instr!(tsx-imp),

            //TXS
            0x9A => instr!(txs-imp),

            //PHA
            0x48 => instr!(pha-imp),

            //PHP
            0x08 => instr!(php-imp),

            //PLA
            0x68 => instr!(pla-imp),

            //PLP
            0x28 => instr!(plp-imp),

            _ => unimplemented!("{:#04X} opcode not implemented yet!\n", opcode),
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

    fn get_sp_addr(&self) -> u16 {
        0x0100 | self.sp as u16
    }

    fn lda(&mut self, addr: u16) {
        self.a = self.ram.read(addr);

        if self.a == 0 {self.set_zero_flag()}
        if self.a & 1 << 7 != 0 {self.set_negative_flag()}

        self.pc += 1;
    }

    fn ldx(&mut self, addr: u16) {
        self.x = self.ram.read(addr);

        if self.x == 0 {self.set_zero_flag()}
        if self.x & 1 << 7 != 0 {self.set_negative_flag()}

        self.pc += 1;
    }

    fn ldy(&mut self, addr: u16) {
        self.y = self.ram.read(addr);

        if self.y == 0 {self.set_zero_flag()}
        if self.y & 1 << 7 != 0 {self.set_negative_flag()}

        self.pc += 1;
    }

    fn sta(&mut self, addr: u16) {
        self.ram.write(addr, self.a);

        self.pc += 1;
    }

    fn stx(&mut self, addr: u16) {
        self.ram.write(addr, self.x);

        self.pc += 1;
    }

    fn sty(&mut self, addr: u16) {
        self.ram.write(addr, self.y);

        self.pc += 1;
    }

    fn tax(&mut self) {
        self.x = self.a;

        if self.x == 0 {self.set_zero_flag()}
        if self.x & 1 << 7 != 0 {self.set_negative_flag()}

        self.pc +=1;
    }

    fn tay(&mut self) {
        self.y = self.a;

        if self.y == 0 {self.set_zero_flag()}
        if self.y & 1 << 7 != 0 {self.set_negative_flag()}

        self.pc +=1;
    }

    fn txa(&mut self) {
        self.a = self.x;

        if self.a == 0 {self.set_zero_flag()}
        if self.a & 1 << 7 != 0 {self.set_negative_flag()}

        self.pc +=1;
    }

    fn tya(&mut self) {
        self.a = self.y;

        if self.a == 0 {self.set_zero_flag()}
        if self.a & 1 << 7 != 0 {self.set_negative_flag()}

        self.pc +=1;
    }

    fn tsx(&mut self) {
        self.x = self.sp;

        if self.x == 0 {self.set_zero_flag()}
        if self.x & 1 << 7 != 0 {self.set_negative_flag()}

        self.pc +=1;
    }

    fn txs(&mut self) {
        self.sp = self.x;

        self.pc +=1;
    }

    fn pha(&mut self) {
        self.ram.write(self.get_sp_addr(), self.a);

        self.pc +=1;
        self.sp -=1;
    }

    fn php(&mut self) {
        self.ram.write(self.get_sp_addr(), self.status);

        self.pc +=1;
        self.sp -=1;
    }

    fn pla(&mut self) {
        self.sp += 1;
        self.a = self.ram.read(self.get_sp_addr());

        if self.a == 0 {self.set_zero_flag()}
        if self.a & 1 << 7 != 0 {self.set_negative_flag()}

        self.pc +=1;
    }

    fn plp(&mut self) {
        self.sp += 1;
        self.status = self.ram.read(self.get_sp_addr());

        self.pc +=1;
    }
}

#[cfg(test)]
mod test {
    use crate::ram::Ram;
    use crate::cpu::Cpu;

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
        let mut cpu = Cpu::create(ram);

        cpu.read_instruction();
        assert!(cpu.get_zero_flag());
    }

    #[test]
    fn lda_negative_flag() {
        let mut ram = Ram::create();
        ram.write(0x0, 0xA9);
        ram.write(0x1, 0x80);
        let mut cpu = Cpu::create(ram);

        cpu.read_instruction();
        assert!(cpu.get_negative_flag());
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
        let mut cpu = Cpu::create(ram);

        cpu.read_instruction();
        assert!(cpu.get_zero_flag());
    }

    #[test]
    fn ldx_negative_flag() {
        let mut ram = Ram::create();
        ram.write(0x0, 0xA2);
        ram.write(0x1, 0x80);
        let mut cpu = Cpu::create(ram);

        cpu.read_instruction();
        assert!(cpu.get_negative_flag());
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
        let mut cpu = Cpu::create(ram);

        cpu.read_instruction();
        assert!(cpu.get_zero_flag());
    }

    #[test]
    fn ldy_negative_flag() {
        let mut ram = Ram::create();
        ram.write(0x0, 0xA0);
        ram.write(0x1, 0x80);
        let mut cpu = Cpu::create(ram);

        cpu.read_instruction();
        assert!(cpu.get_negative_flag());
    }

    #[test]
    fn sta_zp() {
        let mut ram = Ram::create();
        ram.write(0x0, 0x85);
        ram.write(0x1, 0x32);
        let mut cpu = Cpu::create(ram);

        cpu.a = 0x3;
        cpu.read_instruction();

        assert_eq!(cpu.ram.read(0x32), 0x3);
        assert_eq!(cpu.pc, 0x2);
    }

    #[test]
    fn sta_zpx() {
        let mut ram = Ram::create();
        ram.write(0x0, 0x95);
        ram.write(0x1, 0x32);
        let mut cpu = Cpu::create(ram);

        cpu.a = 0x3;
        cpu.x = 0x2;
        cpu.read_instruction();

        assert_eq!(cpu.ram.read(0x32+0x2), 0x3);
        assert_eq!(cpu.pc, 0x2);
    }

    #[test]
    fn sta_abs() {
        let mut ram = Ram::create();
        ram.write(0x0, 0x8D);
        ram.write(0x1, 0x32);
        ram.write(0x2, 0x69);
        let mut cpu = Cpu::create(ram);

        cpu.a = 0x3;
        cpu.read_instruction();

        assert_eq!(cpu.ram.read(0x6932), 0x3);
        assert_eq!(cpu.pc, 0x3);
    }

    #[test]
    fn sta_abx() {
        let mut ram = Ram::create();
        ram.write(0x0, 0x9D);
        ram.write(0x1, 0x32);
        ram.write(0x2, 0x69);
        let mut cpu = Cpu::create(ram);

        cpu.a = 0x3;
        cpu.x = 0x2;
        cpu.read_instruction();

        assert_eq!(cpu.ram.read(0x6932 + 0x2), 0x3);
        assert_eq!(cpu.pc, 0x3);
    }

    #[test]
    fn sta_aby() {
        let mut ram = Ram::create();
        ram.write(0x0, 0x99);
        ram.write(0x1, 0x32);
        ram.write(0x2, 0x69);
        let mut cpu = Cpu::create(ram);

        cpu.a = 0x3;
        cpu.y = 0x2;
        cpu.read_instruction();

        assert_eq!(cpu.ram.read(0x6932 + 0x2), 0x3);
        assert_eq!(cpu.pc, 0x3);
    }

    #[test]
    fn sta_inx() {
        let mut ram = Ram::create();
        ram.write(0x0, 0x81);
        ram.write(0x1, 0x32);
        ram.write(0x32 + 0x2, 0x44);
        ram.write(0x33 + 0x2, 0x64);
        let mut cpu = Cpu::create(ram);

        cpu.a = 0x3;
        cpu.x = 0x2;
        cpu.read_instruction();

        assert_eq!(cpu.ram.read(0x6444), 0x3);
        assert_eq!(cpu.pc, 0x2);
    }

    #[test]
    fn sta_iny() {
        let mut ram = Ram::create();
        ram.write(0x0, 0x91);
        ram.write(0x1, 0x32);
        ram.write(0x32, 0x44);
        ram.write(0x33, 0x64);
        let mut cpu = Cpu::create(ram);

        cpu.a = 0x3;
        cpu.y = 0x2;
        cpu.read_instruction();

        assert_eq!(cpu.ram.read(0x6444 + 0x2), 0x3);
        assert_eq!(cpu.pc, 0x2);
    }

    #[test]
    fn stx_zp() {
        let mut ram = Ram::create();
        ram.write(0x0, 0x86);
        ram.write(0x1, 0x32);
        let mut cpu = Cpu::create(ram);

        cpu.x = 0x3;
        cpu.read_instruction();

        assert_eq!(cpu.ram.read(0x32), 0x3);
        assert_eq!(cpu.pc, 0x2);
    }

    #[test]
    fn stx_zpy() {
        let mut ram = Ram::create();
        ram.write(0x0, 0x96);
        ram.write(0x1, 0x32);
        let mut cpu = Cpu::create(ram);

        cpu.x = 0x3;
        cpu.y = 0x2;
        cpu.read_instruction();

        assert_eq!(cpu.ram.read(0x32+0x2), 0x3);
        assert_eq!(cpu.pc, 0x2);
    }

    #[test]
    fn stx_abs() {
        let mut ram = Ram::create();
        ram.write(0x0, 0x8E);
        ram.write(0x1, 0x32);
        ram.write(0x2, 0x69);
        let mut cpu = Cpu::create(ram);

        cpu.x = 0x3;
        cpu.read_instruction();

        assert_eq!(cpu.ram.read(0x6932), 0x3);
        assert_eq!(cpu.pc, 0x3);
    }

    #[test]
    fn sty_zp() {
        let mut ram = Ram::create();
        ram.write(0x0, 0x84);
        ram.write(0x1, 0x32);
        let mut cpu = Cpu::create(ram);

        cpu.y = 0x3;
        cpu.read_instruction();

        assert_eq!(cpu.ram.read(0x32), 0x3);
        assert_eq!(cpu.pc, 0x2);
    }

    #[test]
    fn sty_zpx() {
        let mut ram = Ram::create();
        ram.write(0x0, 0x94);
        ram.write(0x1, 0x32);
        let mut cpu = Cpu::create(ram);

        cpu.y = 0x3;
        cpu.x = 0x2;
        cpu.read_instruction();

        assert_eq!(cpu.ram.read(0x32+0x2), 0x3);
        assert_eq!(cpu.pc, 0x2);
    }

    #[test]
    fn sty_abs() {
        let mut ram = Ram::create();
        ram.write(0x0, 0x8C);
        ram.write(0x1, 0x32);
        ram.write(0x2, 0x69);
        let mut cpu = Cpu::create(ram);

        cpu.y = 0x3;
        cpu.read_instruction();

        assert_eq!(cpu.ram.read(0x6932), 0x3);
        assert_eq!(cpu.pc, 0x3);
    }

    #[test]
    fn tax_imp() {
        let mut ram = Ram::create();
        ram.write(0x0, 0xAA);
        let mut cpu = Cpu::create(ram);

        cpu.a = 0x3;
        cpu.read_instruction();

        assert_eq!(cpu.x, cpu.a);
        assert_eq!(cpu.pc, 0x1);
    }

    #[test]
    fn tax_zero_flag() {
        let mut ram = Ram::create();
        ram.write(0x0, 0xAA);
        let mut cpu = Cpu::create(ram);

        cpu.a = 0x0;
        cpu.read_instruction();

        assert!(cpu.get_zero_flag());
    }

    #[test]
    fn tax_negative_flag() {
        let mut ram = Ram::create();
        ram.write(0x0, 0xAA);
        let mut cpu = Cpu::create(ram);

        cpu.a = 0x80;
        cpu.read_instruction();

        assert!(cpu.get_negative_flag());
    }

    #[test]
    fn tay_imp() {
        let mut ram = Ram::create();
        ram.write(0x0, 0xA8);
        let mut cpu = Cpu::create(ram);

        cpu.a = 0x3;
        cpu.read_instruction();

        assert_eq!(cpu.y, cpu.a);
        assert_eq!(cpu.pc, 0x1);
    }

    #[test]
    fn tay_zero_flag() {
        let mut ram = Ram::create();
        ram.write(0x0, 0xA8);
        let mut cpu = Cpu::create(ram);

        cpu.a = 0x0;
        cpu.read_instruction();

        assert!(cpu.get_zero_flag());
    }

    #[test]
    fn tay_negative_flag() {
        let mut ram = Ram::create();
        ram.write(0x0, 0xA8);
        let mut cpu = Cpu::create(ram);

        cpu.a = 0x80;
        cpu.read_instruction();

        assert!(cpu.get_negative_flag());
    }

    #[test]
    fn txa_imp() {
        let mut ram = Ram::create();
        ram.write(0x0, 0x8A);
        let mut cpu = Cpu::create(ram);

        cpu.x = 0x3;
        cpu.read_instruction();

        assert_eq!(cpu.a, cpu.x);
        assert_eq!(cpu.pc, 0x1);
    }

    #[test]
    fn txa_zero_flag() {
        let mut ram = Ram::create();
        ram.write(0x0, 0x8A);
        let mut cpu = Cpu::create(ram);

        cpu.x = 0x0;
        cpu.read_instruction();

        assert!(cpu.get_zero_flag());
    }

    #[test]
    fn txa_negative_flag() {
        let mut ram = Ram::create();
        ram.write(0x0, 0x8A);
        let mut cpu = Cpu::create(ram);

        cpu.x = 0x80;
        cpu.read_instruction();

        assert!(cpu.get_negative_flag());
    }

    #[test]
    fn tya_imp() {
        let mut ram = Ram::create();
        ram.write(0x0, 0x98);
        let mut cpu = Cpu::create(ram);

        cpu.y = 0x3;
        cpu.read_instruction();

        assert_eq!(cpu.a, cpu.y);
        assert_eq!(cpu.pc, 0x1);
    }

    #[test]
    fn tya_zero_flag() {
        let mut ram = Ram::create();
        ram.write(0x0, 0x98);
        let mut cpu = Cpu::create(ram);

        cpu.y = 0x0;
        cpu.read_instruction();

        assert!(cpu.get_zero_flag());
    }

    #[test]
    fn tya_negative_flag() {
        let mut ram = Ram::create();
        ram.write(0x0, 0x98);
        let mut cpu = Cpu::create(ram);

        cpu.y = 0x80;
        cpu.read_instruction();

        assert!(cpu.get_negative_flag());
    }

    #[test]
    fn tsx_imp() {
        let mut ram = Ram::create();
        ram.write(0x0, 0xBA);
        let mut cpu = Cpu::create(ram);

        cpu.read_instruction();

        assert_eq!(cpu.x, cpu.sp);
        assert_eq!(cpu.pc, 0x1);
    }

    #[test]
    fn tsx_zero_flag() {
        let mut ram = Ram::create();
        ram.write(0x0, 0xBA);
        let mut cpu = Cpu::create(ram);

        cpu.sp = 0x00;
        cpu.read_instruction();

        assert!(cpu.get_zero_flag());
    }

    #[test]
    fn tsx_negative_flag() {
        let mut ram = Ram::create();
        ram.write(0x0, 0xBA);
        let mut cpu = Cpu::create(ram);

        cpu.sp = 0x80;
        cpu.read_instruction();

        assert!(cpu.get_negative_flag());
    }

    #[test]
    fn txs_imp() {
        let mut ram = Ram::create();
        ram.write(0x0, 0x9A);
        let mut cpu = Cpu::create(ram);

        cpu.x = 0x69;
        cpu.read_instruction();

        assert_eq!(cpu.sp, cpu.x);
        assert_eq!(cpu.pc, 0x01);
    }

    #[test]
    fn pha_imp() {
        let mut ram = Ram::create();
        ram.write(0x0, 0x48);
        let mut cpu = Cpu::create(ram);

        cpu.a = 0x69;
        cpu.read_instruction();

        assert_eq!(cpu.ram.read(0x01ff), 0x69);
        assert_eq!(cpu.sp, 0xfe);
        assert_eq!(cpu.pc, 0x01);
    }

    #[test]
    fn php_imp() {
        let mut ram = Ram::create();
        ram.write(0x0, 0x08);
        let mut cpu = Cpu::create(ram);

        cpu.status = 0x69;
        cpu.read_instruction();

        assert_eq!(cpu.ram.read(0x01ff), 0x69);
        assert_eq!(cpu.sp, 0xfe);
        assert_eq!(cpu.pc, 0x01);
    }

    #[test]
    fn pla_zero_flag() {
        let mut ram = Ram::create();
        ram.write(0x0, 0x68);
        ram.write(0x01ff, 0x00);
        let mut cpu = Cpu::create(ram);

        cpu.sp = 0xfe;
        cpu.read_instruction();

        assert!(cpu.get_zero_flag());
    }

    #[test]
    fn pla_negative_flag() {
        let mut ram = Ram::create();
        ram.write(0x0, 0x68);
        ram.write(0x01ff, 0x80);
        let mut cpu = Cpu::create(ram);

        cpu.sp = 0xfe;
        cpu.read_instruction();

        assert!(cpu.get_negative_flag());
    }

    #[test]
    fn pla_imp() {
        let mut ram = Ram::create();
        ram.write(0x0, 0x68);
        ram.write(0x01ff, 0x42);
        let mut cpu = Cpu::create(ram);

        cpu.sp = 0xfe;
        cpu.read_instruction();

        assert_eq!(cpu.a, 0x42);
        assert_eq!(cpu.sp, 0xff);
        assert_eq!(cpu.pc, 0x01);
    }

    #[test]
    fn plp_imp() {
        let mut ram = Ram::create();
        ram.write(0x0, 0x28);
        ram.write(0x01ff, 0x42);
        let mut cpu = Cpu::create(ram);

        cpu.sp = 0xfe;
        cpu.read_instruction();

        assert_eq!(cpu.status, 0x42);
        assert_eq!(cpu.sp, 0xff);
        assert_eq!(cpu.pc, 0x01);
    }
}
