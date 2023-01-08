const RAM_SIZE: usize = 1024*64;

pub struct Ram {
    memory : [u8; RAM_SIZE],
}

impl Ram {
    pub fn create() -> Ram {
        Ram { memory: ( [0; RAM_SIZE] ) }
    }

    pub fn write(&mut self, addr : u16 ,value : u8) {
        self.memory[addr as usize] = value;
    }

    pub fn read(&mut self, addr : u16) -> u8 {
        self.memory[addr as usize]
    }
}
