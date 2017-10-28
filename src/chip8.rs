extern crate rand;

pub struct Chip8 {
    memory: [u8; 4096],
    register: [u8; 16],
    index: u16,
    pc: u16,
    pub display: [bool; 64*32],
    stack: [u16; 16],
    sp: u16,
    opcode: u16,
    delay_timer: u8,
    sound_timer: u8,
    pub draw_flag: bool,
}

const FONT_SET:  [u8; 80] = [
    0xF0,0x90,0x90,0x90,0xF0, //0
    0x20,0x60,0x20,0x20,0x70, //1
    0xF0,0x10,0xF0,0x80,0xF0, //2
    0xF0,0x10,0xF0,0x10,0xF0, //3
    0x90,0x90,0xF0,0x10,0x10, //4
    0xF0,0x80,0xF0,0x10,0xF0, //5
    0xF0,0x80,0xF0,0x90,0xF0, //6
    0xF0,0x10,0x20,0x40,0x40, //7
    0xF0,0x90,0xF0,0x90,0xF0, //8
    0xF0,0x90,0xF0,0x10,0xF0, //9
    0xF0,0x90,0xF0,0x90,0x90, //A
    0xE0,0x90,0xE0,0x90,0xE0, //B
    0xF0,0x80,0x80,0x80,0xF0, //C
    0xE0,0x90,0x90,0x90,0xE0, //D
    0xF0,0x80,0xF0,0x80,0xF0, //E
    0xF0,0x80,0xF0,0x80,0x80, //F
    ];

impl Chip8 {
    pub fn new(op_code: Vec<u8>, is_test: bool) ->  Chip8 {
        let mut memory: [u8; 4096] = [0; 4096];
        
        if !is_test {
            for (i, byte) in op_code.iter().enumerate() {
                memory[0x200 + i] = byte.clone();
            }

            for (i, byte) in FONT_SET.iter().enumerate() {
                memory[i]  = byte.clone();
            }
        }
        else {
           for (i, byte) in FONT_SET.iter().enumerate() {
               memory[i]  = byte.clone();
           }

           memory[0x200 ] = 0xD1;
           memory[0x200 + 1] = 0x25;
           memory[0x200 + 2] = 0xA0;
           memory[0x200 + 3] = 0x05;
           memory[0x200 + 4] = 0x61;
           memory[0x200 + 5] = 0x05;
           memory[0x200 + 6] = 0xD1;
           memory[0x200 + 7] = 0x25;
        }

        Chip8 {
            memory,
            register: [0; 16],
            index: 0,
            pc: 0x200,
            display: [false; 64*32],
            stack: [0; 16],
            sp: 0,
            opcode: 0,
            delay_timer: 0,
            sound_timer: 0,
            draw_flag: false,
        }
    }
    
    pub fn run_cycle(&mut self) {
            self.fetch_opcode();
            if self.opcode != 0 {
                println!("V[1]: {}, V[2]: {}", self.register[1],self.register[2]);
                if self.draw_flag {
                    let mut i = 0;
                    while i < 64*32 {
                        if self.display[i] == true {
                            println!("display=true at: {}", i) 
                        }
                        i += 1;
                    }
                    self.draw_flag = false;
                }
                println!("Executed {:X}!", self.opcode);
                self.run_opcode();
            }
    }
    
    fn fetch_opcode(&mut self) {
        self.opcode = ((self.memory[self.pc as usize] as u16) << 8) 
        | self.memory[(self.pc + 1) as usize] as u16;
    }
    
    fn run_opcode(&mut self) {
        match self.opcode & 0xF000 {
            0x0000 => self.op_0xxx(),
            0x1000 => self.op_1xxx(),
            0x2000 => self.op_2xxx(),
            0x3000 => self.op_3xxx(),
            0x4000 => self.op_4xxx(),
            0x5000 => self.op_5xxx(),
            0x6000 => self.op_6xxx(),
            0x7000 => self.op_7xxx(),
            0x8000 => self.op_8xxx(),
            0x9000 => self.op_9xxx(),
            0xA000 => self.op_axxx(),
            0xB000 => self.op_bxxx(),
            0xC000 => self.op_cxxx(),
            0xD000 => self.op_dxxx(),
            0xF000 => self.op_fxxx(),
            _ => {
                println!("opcode: {:X},not implemented yet", self.opcode);
                self.pc += 2;
            }
        }
    }
 
    fn op_0xxx(&mut self) {
        match self.opcode & 0xFF00 {
            0x0000 => self.op_00xx(),
            _ => {
                println!("{} not implemented yet!!!", self.opcode);
                self.pc += 2;
            }
        }
    }

    fn op_00xx(&mut self) {
        match self.opcode & 0x00FF {
            0x00E0 => {
                self.display = [false; 64*32];
                self.pc += 2;
            }
            
            _ => {
                println!("{} not implemented yet!!!", self.opcode);
                self.pc += 2;
            }

        }
    }

    fn op_1xxx(&mut self) {
        //1NNN: Jump to the address NNN
        self.pc = self.opcode & 0x0FFF;
    }
    
    fn op_2xxx(&mut self) {
        //2NNN: call subroutine at NNN -> store pc on stack and jump to address NNN
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = self.opcode & 0x0FFF;
    }
    
    fn op_3xxx(&mut self) {
        //3XNN: skip next instruction if V[X] == NN
        let x: u16 = self.register[((self.opcode & 0x0F00) >> 8) as usize] as u16;
        let y: u16 = (self.opcode & 0x00FF) as u16;
        if x == y {
            self.pc += 4; 
        }
        else {
            self.pc += 2;
        }
    }
    
    fn op_4xxx(&mut self) {
        //4XNN: skip the next instruction if V[X] != NN
        let x: u16 = self.register[((self.opcode & 0x0F00) >> 8) as usize] as u16;
        let y: u16 = (self.opcode & 0x00FF) as u16;
        if x != y {
            self.pc += 4;
        }
        else {
            self.pc += 2;
        }
    }
    
    fn op_5xxx(&mut self) {
        //5XY0: skip thenext instruction if V[X] == V[Y] 
        let x: u16 = self.register[((self.opcode & 0x0F00) >> 8) as usize] as u16;
        let y: u16 = self.register[((self.opcode & 0x00F0) >> 4) as usize] as u16;
        if x == y {
            self.pc += 4; 
        }
        else {
            self.pc += 2;
        }
    }
    
    fn op_6xxx(&mut self) {
        //6XNN: sets V[X] to NN
        self.register[((self.opcode & 0x0F00) >> 8) as usize] = (self.opcode & 0x00FF) as u8;
        self.pc += 2; 
    }
    
    fn op_7xxx(&mut self) {
        //7XNN: add NN to V[X]
        self.register[((self.opcode & 0x0F00) >> 8) as usize] += (self.opcode & 0x00FF) as u8;
        self.pc += 2;
    }
    
    fn op_8xxx(&mut self) {
        match self.opcode & 0x000F {
            0x0000 => {
                //8XY0: set V[X] = V[Y]
                self.register[((self.opcode & 0x0F00) >> 8) as usize] = self.register[((self.opcode & 0x00F0) >> 4) as usize];
                self.pc += 2;
            } 
            
            0x0001 => {
                //8XY1: set V[X] = (V[X] or V[Y])
                self.register[((self.opcode & 0x0F00) >> 8) as usize] |=
                    self.register[((self.opcode & 0x00F0) >>  4) as usize];
                self.pc += 2;
            }
            
            0x0002 => {
                //8XY2: set V[X] = (V[X] and V[Y])
                self.register[((self.opcode & 0x0F00) >> 8) as usize] &=
                    self.register[((self.opcode & 0x00F0) >>  4) as usize];
                self.pc += 2;
            }
            
            0x0003 => {
                //8XY3: set V[X] = (V[X] xor V[Y])
                self.register[((self.opcode & 0x0F00) >> 8) as usize] ^= 
                    self.register[((self.opcode & 0x00F0) >>  4) as usize];
                self.pc += 2;
            }
            
            0x0004 => {
                //8XY4: add V[Y] to V[X], if carry set V[F] = 1, if no carry set V[F] = 0
                self.register[15] =
                    if self.register[((self.opcode & 0x00F0) >> 4) as usize] >
                       (0xFF - self.register[((self.opcode & 0x0F00) >> 8) as usize])
                    {1} else {0};
                self.register[((self.opcode & 0x0F00) >> 8) as usize] +=
                    self.register[((self.opcode & 0x00F0) >> 4) as usize];
                self.pc += 2;
            }
            
            0x0005 => {
                //8XY5: set V[X] -= V[Y], if borrow set V[F] = 0, else set V[F] = 1
                self.register[15] =
                    if self.register[((self.opcode & 0x00F0) >> 4) as usize] >
                        self.register[((self.opcode & 0x0F00) >> 8) as usize]
                    {1} else {0};
                self.register[((self.opcode & 0x0F00) >> 8) as usize] -= self.register[((self.opcode & 0x00F0) >> 4) as usize];
                self.pc += 2;
            }
            
            0x0006 => {
                //8XY6: set V[F] to LSB of V[Y], set V[X] = (V[Y] >> 1)
                self.register[15] = self.register[((self.opcode & 0x00F0) >> 4) as usize] & 0x1;
                self.register[((self.opcode & 0x0F00) >> 8) as usize] = self.register[((self.opcode & 0x00F0) >> 4) as usize] >> 1;
                self.pc += 2;
            }
            
            0x0007 => {
                //8XY7: set V[X] = (V[Y] - V[X]), if borrow set V[F] = 0, else set V[F] = 1
                self.register[15] =
                    if self.register[((self.opcode & 0x0F00) >> 8) as usize] >
                        self.register[((self.opcode & 0x00F0) >> 4) as usize]
                    {1} else {0};
                self.register[((self.opcode & 0x0F00) >> 8) as usize] =
                    self.register[((self.opcode & 0x00F0) >> 4) as usize] - self.register[((self.opcode & 0x0F00) >> 8) as usize];
                self.pc += 2;
            }
            
            0x000E => {
                //8XYE: set V[F] to MSB of V[Y], set V[X] = (V[Y] << 1)
                self.register[15] =
                    if  (self.register[((self.opcode & 0x00F0) >> 4) as usize] as u16 & 0x8000 as u16) > 0
                    {1} else {0};
                self.register[((self.opcode & 0x0F00) >> 8) as usize] = self.register[((self.opcode & 0x00F0) >> 4) as usize] << 1;
                self.pc += 2;
            
            }
            _ => {
                println!("opcode: {:X},not implemented yet", self.opcode);
                self.pc += 2;
            }
        }
    } 

    fn op_9xxx(&mut self) {
        //9XY0: skip the next instruction if V[X] != V[Y]
        let x: u16 = self.register[((self.opcode & 0x0F00) >> 8) as usize] as u16;
        let y: u16 = self.register[((self.opcode & 0x00F0) >> 4) as usize] as u16; 
        if x != y {
            self.pc += 4;
        }
        else {
            self.pc += 2;
        }
    }
    
    fn op_axxx(&mut self) {
        //ANNN: sets the index to the adress NNN
        self.index = self.opcode & 0x0FFF;
        self.pc += 2;
    }
    
    fn op_bxxx(&mut self) {
        //BNNN: jump to the address V[0] + NNN
        self.pc = self.register[0] as u16 + self.opcode & 0x0FFF;
    }
    
    fn op_cxxx(&mut self) {
        //CXNN: set V[X] to random u8 and NN
        self.register[((self.opcode & 0x0F00) >> 8) as usize] = rand::random::<(u8)>() & self.register[(self.opcode & 0x00FF) as usize];
        self.pc += 2;
    }
    
    fn op_dxxx(&mut self) {
        //DXYN: draw sprite at coordinate (V[X],V[Y]) 
        //      with a width of 8 pixels and a hight of N pixels
        let x = self.register[((self.opcode & 0x0F00) >> 8) as usize] as u16;
        let y = self.register[((self.opcode & 0x00F0) >> 4) as usize] as u16;
        let hight = self.opcode & 0x000F;
        let mut font_row: u8;

        self.register[15] = 0;

        for row in 0..hight {
            font_row = self.memory[(self.index + row) as usize];

            for column in 0..8 {
                //this checks for every column/pixel in this row if it equals 0
                if font_row & (0x80 >> column) != 0 {
                    if self.display[(x + column + ((y + row) * 64)) as usize] == true {
                        self.register[15] = 1;
                    }
                    self.display[(x + column + ((y + row) * 64)) as usize] ^= true;
                }
            }
        }
        self.draw_flag = true;
        self.pc +=2;
    }
    
    
    fn op_fxxx(&mut self) {
        match self.opcode & 0x00FF {
            0x0007 => {
                //FX07:set V[X] to delay_timer
                self.register[((self.opcode & 0x0F00) >> 8) as usize] = self.delay_timer;
                self.pc += 2;
            }
            
            0x0015 => {
                //FX15: set delay_timer to V[X]
                self.delay_timer = self.register[((self.opcode & 0x0F00) >> 8) as usize] as u8;
                self.pc += 2;
            }
            
            0x0018 => {
                //FX18: set sound_timer to V[X]
                self.sound_timer = self.register[((self.opcode & 0x0F00) >> 8) as usize] as u8;
                self.pc += 2;
            }
            0x001E => {
                //FX1E: add V[X] to I
                self.index += self.register[((self.opcode & 0x0F00) >> 8) as usize] as u16;
                self.pc += 2;
            }

            0x0029 => {
                //FX29: set I to the location ofthe sprite for the character in V[X]
                let sprite: u8 = self.register[((self.opcode & 0x0F00) >> 8) as usize];
                self.index = (sprite *5) as u16;
                self.pc += 2;
            }

            0x0033 => {
                //FX33: store the BCD of V[X] in memory as following: M[I] = V[X](3), M[I+1] = V[X](2), M[I+2] = V[X](1)
                self.memory[self.index as usize] = self.register[((self.opcode & 0x0F00) >> 8) as usize] / 100;
                self.memory[(self.index + 1) as usize] = (self.register[((self.opcode & 0x0F00) >> 8) as usize] % 100) / 10;
                self.memory[(self.index + 2) as usize] = self.register[((self.opcode & 0x0F00) >> 8) as usize] % 10;
                self.pc += 2;
            }
            
            0x0055 => {
                //FX55: store V[0] to V[X] in memory starting with I
                for x in 0..((self.opcode & 0x0F00) >> 8) {
                    self.memory[self.index as usize] = self.register[x as usize];
                    self.index += 1;
                }
                self.pc += 2;
            }
            
            0x0065 => {
                //FX65: store memory starting with I in V[0] to V[X]
                for x in 0..((self.opcode & 0x0F00) >> 8) {
                    self.register[x as usize] = self.memory[self.index as usize];
                    self.index += 1;
                }
                self.pc += 2;
            }
                   
            _ => {
                println!("opcode: {:X}, not implemented yet", self.opcode);
                self.pc += 2;
            }
        }
    }
}
