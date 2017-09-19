pub struct Chip8
{
  memory: [u8; 4096],
  register: [u8; 16],
  index: u16,
  pc: u16,
  display: [u8; 30*64],
  stack: [u16; 16],
  sp: u16,
  opcode: u16,
}

impl Chip8
{
  pub fn new(op_code: Vec<u8>) ->  Chip8
  {
    let mut memory: [u8; 4096] = [0; 4096];
    
    for (i, byte) in op_code.iter().enumerate()
    {
      memory[0x200 + i] = byte.clone();
    }
    Chip8
    {
      memory,
      register: [0; 16],
      index: 0,
      pc: 0x200,
      display: [0; 30*64],
      stack: [0; 16],
      sp: 0,
      opcode: 0
    }
  }

  pub fn run_cycle(&mut self)
  {
    for x in 0..30
    {
      self.fetch_opcode();
      self.run_opcode();
    }
  }

  fn fetch_opcode(&mut self)
  {
    self.opcode = (self.memory[self.pc as usize] as u16) << 8 
    | self.memory[(self.pc + 1) as usize] as u16;
  }

  fn run_opcode(&mut self)
  {
    match self.opcode & 0xF000
    {
      0x1000 => self.op_1xxx(),
      0x2000 => self.op_2xxx(),
      0x3000 => self.op_3xxx(),
      0x4000 => self.op_4xxx(),
      0x5000 => self.op_5xxx(),
      0x6000 => self.op_6xxx(),
      0x9000 => self.op_9xxx(),
      0xA000 => self.op_Axxx(),
      0xD000 => self.op_Dxxx(),
      _ => {
        println!("opcode: {:X},not implemented yet", self.opcode);
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
    let x: u16 = self.register[(self.opcode & 0x0F00 >> 8) as usize] as u16;
    let y: u16 = (self.opcode & 0x00FF) as u16;
    
    if x == y
    {
      self.pc += 4; 
    }
    else
    {
      self.pc += 2;
    }
 }

 fn op_4xxx(&mut self) {
   //4XNN: skip the next instruction if V[X] != NN
   let x: u16 = self.register[(self.opcode & 0x0F00 >> 8) as usize] as u16;
   let y: u16 = (self.opcode & 0x00FF) as u16;
   if x != y
   {
    self.pc += 4;
   }
   else
   {
    self.pc += 2;
   }
 }

 fn op_5xxx(&mut self) {
  //5XY0: skip thenext instruction if V[X] == V[Y] 
  let x: u16 = self.register[(self.opcode & 0x0F00 >> 8) as usize] as u16;
  let y: u16 = self.register[(self.opcode & 0x00F0 >> 4) as usize] as u16;

  if x == y
  {
    self.pc += 4; 
  }
  else
  {
    self.pc += 2;
  }
 }

 fn op_6xxx(&mut self) {
  //6XNN: sets V[X] to NN
  self.register[(self.opcode & 0x0F00 >> 8) as usize] = (self.opcode & 0x00FF) as u8;
  self.pc += 2; 
 }

 fn op_9xxx(&mut self) {
  //9XY0: skip the next instruction if V[X] != V[Y]
  let x: u16 = self.register[(self.opcode & 0x0F00 >> 8) as usize] as u16;
  let y: u16 = self.register[(self.opcode & 0x00F0 >> 4) as usize] as u16;
  
  if x != y
  { 
    self.pc += 4;
  }
  else
  { 
    self.pc += 2;
  }
 }

 fn op_Axxx(&mut self) {
   //ANNN: sets the index to the adress NNN
   self.index = self.opcode & 0x0FFF;
   self.pc += 2;
 }

 fn op_Dxxx(&mut self) {
   //Waiting with the drawing stuff until later, just increase pc for now
   println!("opcode: {:X}, not implemented yet", self.opcode);
   self.pc += 2;
 }
}

