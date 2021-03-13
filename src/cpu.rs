use rand::distributions::{Distribution, Uniform};

const WIDTH: usize = 64;                                                                                                const HEIGHT: usize = 32;                                                                                                                                                                                                                       pub const FONTSET: [u8; 80] = [                                                                                             0xF0, 0x90, 0x90, 0x90, 0xF0,                                                                                           0x20, 0x60, 0x20, 0x20, 0x70,                                                                                           0x20, 0x60, 0x20, 0x20, 0x70,                                                                                           0xF0, 0x10, 0xF0, 0x10, 0xF0,                                                                                           0x90, 0x90, 0xF0, 0x10, 0x10,                                                                                           0xF0, 0x80, 0xF0, 0x10, 0xF0,                                                                                           0xF0, 0x80, 0xF0, 0x90, 0xF0,                                                                                           0xF0, 0x10, 0x20, 0x40, 0x40,                                                                                           0xF0, 0x90, 0xF0, 0x90, 0xF0,                                                                                           0xF0, 0x90, 0xF0, 0x10, 0xF0,                                                                                           0xF0, 0x90, 0xF0, 0x90, 0x90,                                                                                           0xE0, 0x90, 0xE0, 0x90, 0xE0,                                                                                           0xF0, 0x80, 0x80, 0x80, 0xF0,                                                                                           0xE0, 0x90, 0x90, 0x90, 0xE0,                                                                                           0xF0, 0x80, 0xF0, 0x80, 0xF0,                                                                                           0xF0, 0x80, 0xF0, 0x80, 0x80                                                                                        ];

pub struct Cpu {
    memory: [u8; 4096], 
    opcode: u16, 
    V: [u8; 16],
    I: u16,
    pc: u16,
    stack: [u16; 16],
    sp: u16,
    gfx: [u8; WIDTH * HEIGHT],
    delay_timer: u8,
    sound_timer: u8,
    keypad: [u8; 16]    
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            memory: [0; 4096],
            opcode: 0,
            V: [0; 16],
            I: 0,
            pc: 0,
            stack: [0; 16], 
            sp: 0,
            gfx: [0; WIDTH * HEIGHT],
            delay_timer: 0,
            sound_timer: 0,
            keypad: [0; 16]
        }
    }

    pub fn reset(&mut self) {
        self.memory = [0; 4096]; 
        self.opcode = 0;
        self.I = 0;
        self.pc = 0x200; 
        self.stack = [0; 16];
        self.sp = 0;
        self.gfx = [0; WIDTH * HEIGHT];
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.keypad = [0; 16];
        
        for i in 0..80 {self.memory[i] = FONTSET[i]};
    }

    fn fetch_opcode(&mut self) {
        self.opcode = (self.memory[self.pc as usize] << 8) as u16 | 
                       self.memory[(self.pc + 1) as usize] as u16
    }

    fn process_opcode(&self) -> fn(&mut Cpu) {
        match self.opcode & 0xF000 {
            0x0000 =>
                match self.opcode & 0x0FFF {
                    0x00E0 => op_00E0,
                    0x00EE => op_00EE,
                    _ => null_op,
                }
            0x1000 => op_1nnn,
            0x2000 => op_2nnn,
            0x3000 => op_3xkk, 
            0x4000 => op_4xkk,
            0x5000 => op_5xy0, 
            0x6000 => op_6xkk,
            0x7000 => op_7xkk, 
            0x8000 =>
                match self.opcode & 0x000F {
                    0x0000 => op_8xy0,
                    0x0001 => op_8xy1,
                    0x0002 => op_8xy2,
                    0x0003 => op_8xy3,
                    0x0004 => op_8xy4,
                    0x0005 => op_8xy5,
                    0x0006 => op_8xy6,
                    0x0007 => op_8xy7,
                    0x000E => op_8xyE,
                    _ => null_op,
                }
            0x9000 => op_9xy0,
            0xA000 => op_Annn, 
            0xB000 => op_Bnnn,
            0xC000 => op_Cxkk,
            0xD000 => op_Dxyn,
            0xE000 => 
                match self.opcode & 0x00FF {
                    0x009E => op_Ex9E,
                    0x00A1 => op_ExA1,
                    _ => null_op,
                }
            0xF000 => 
                match self.opcode & 0x00FF {
                    0x0007 => op_Fx07,
                    0x000A => op_Fx0A,
                    0x0015 => op_Fx15,
                    0x0018 => op_Fx18,
                    0x001E => op_Fx1E, 
                    0x0029 => op_Fx29,
                    0x0033 => op_Fx33,
                    0x0055 => op_Fx55,
                    0x0065 => op_Fx65,
                    _ => null_op,
                }
            _ => null_op,
        }
    }

    fn execute_opcode(&mut self, action: fn(&mut Cpu)) {
        self.pc += 2;

        action(self)
    }

    fn emulate_cycle(&mut self) {
        self.fetch_opcode();
        self.execute_opcode(self.process_opcode());

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }
}

fn null_op(cpu: &mut Cpu) {
}

fn op_00E0(cpu: &mut Cpu) {
    cpu.gfx = [0; WIDTH * HEIGHT] 
}

fn op_00EE(cpu: &mut Cpu) {
    cpu.sp -= 1;
    cpu.pc = cpu.stack[cpu.sp as usize];
}

fn op_1nnn(cpu: &mut Cpu) {
    cpu.pc = cpu.opcode & 0x0FFF;
}

fn op_2nnn(cpu: &mut Cpu) {
    cpu.stack[cpu.sp as usize] = cpu.pc;
    cpu.sp += 1;
    cpu.pc = cpu.opcode & 0x0FFF;
}

fn op_3xkk(cpu: &mut Cpu) {
    let x = ((cpu.opcode & 0x0F00) >> 8) as usize;
    let byte = (cpu.opcode & 0x00FF) as u8;

    if cpu.V[x] == byte {
        cpu.pc += 2;
    }
}

fn op_4xkk(cpu: &mut Cpu) {
    let x = ((cpu.opcode & 0x0F00) >> 8) as usize;
    let byte = (cpu.opcode & 0x00FF) as u8;

    if cpu.V[x] != byte {
        cpu.pc += 2;
    } 
}

fn op_5xy0(cpu: &mut Cpu) {
    let x = ((cpu.opcode & 0x0F00) >> 8) as usize;
    let y = ((cpu.opcode & 0x0F00) >> 4) as usize;

    if cpu.V[x] == cpu.V[y] {
        cpu.pc += 2;
    }
}

fn op_6xkk(cpu: &mut Cpu) {
    let x = ((cpu.opcode & 0x0F00) >> 8) as usize;
    let byte = (cpu.opcode & 0x00FF) as u8;

    cpu.V[x] = byte;
}

fn op_7xkk(cpu: &mut Cpu) {
    let x = ((cpu.opcode & 0x0F00) >> 8) as usize;
    let byte = (cpu.opcode & 0x00FF) as u8;

    cpu.V[x] += byte;
}

fn op_8xy0(cpu: &mut Cpu) {
    let x = ((cpu.opcode & 0x0F00) >> 8) as usize;
    let y = ((cpu.opcode & 0x0F00) >> 4) as usize;

    cpu.V[x] = cpu.V[y]; 
}

fn op_8xy1(cpu: &mut Cpu) {
    let x = ((cpu.opcode & 0x0F00) >> 8) as usize;
    let y = ((cpu.opcode & 0x0F00) >> 4) as usize;

    cpu.V[x] |= cpu.V[y];
}

fn op_8xy2(cpu: &mut Cpu) {
    let x = ((cpu.opcode & 0x0F00) >> 8) as usize;
    let y = ((cpu.opcode & 0x0F00) >> 4) as usize;

    cpu.V[x] &= cpu.V[y];
}

fn op_8xy3(cpu: &mut Cpu) {
    let x = ((cpu.opcode & 0x0F00) >> 8) as usize;
    let y = ((cpu.opcode & 0x0F00) >> 4) as usize;

    cpu.V[x] ^= cpu.V[y];
}

fn op_8xy4(cpu: &mut Cpu) {
    let x = ((cpu.opcode & 0x0F00) >> 8) as usize;
    let y = ((cpu.opcode & 0x0F00) >> 4) as usize;

    let sum = cpu.V[x] + cpu.V[y]; 

    if sum > 255 {
        cpu.V[0xF] = 1;
    } else {
        cpu.V[0xF] = 0;
    }

    cpu.V[x] = sum & 0xFF
}

fn op_8xy5(cpu: &mut Cpu) {
    let x = ((cpu.opcode & 0x0F00) >> 8) as usize;
    let y = ((cpu.opcode & 0x0F00) >> 4) as usize;

    if cpu.V[x] > cpu.V[y] {
        cpu.V[0xF] = 1;
    } else {
        cpu.V[0xF] = 0;
    }

    cpu.V[x] -= cpu.V[y];
}

fn op_8xy6(cpu: &mut Cpu) {
    let x = ((cpu.opcode & 0x0F00) >> 8) as usize;

    cpu.V[0xF] = cpu.V[x] & 0x1;
    cpu.V[x] >>= 1;
}

fn op_8xy7(cpu: &mut Cpu) {
    let x = ((cpu.opcode & 0x0F00) >> 8) as usize;                                                                                                                              let y = ((cpu.opcode & 0x0F00) >> 4) as usize; 

    if cpu.V[x] > cpu.V[y] {                                                                                                                                                        cpu.V[0xF] = 1;                                                                                                                                                         } else {                                                                                                                                                                        cpu.V[0xF] = 0;                                                                                                                                                         }     

    cpu.V[x] = cpu.V[y] - cpu.V[x];
}

fn op_8xyE(cpu: &mut Cpu) {
    let x = ((cpu.opcode & 0x0F00) >> 8) as usize; 

    cpu.V[0xF] = (cpu.V[x] & 0x80) >> 7;
    cpu.V[x] <<= 1;
}

fn op_9xy0(cpu: &mut Cpu) {
    let x = ((cpu.opcode & 0x0F00) >> 8) as usize; 
    let y = ((cpu.opcode & 0x0F00) >> 4) as usize; 

    if cpu.V[x] != cpu.V[y] {
        cpu.pc += 2;
    }
}

fn op_Annn(cpu: &mut Cpu) {
    cpu.I = cpu.opcode & 0x0FFF;
}

fn op_Bnnn(cpu: &mut Cpu) {
    cpu.pc = cpu.V[0] as u16 + (cpu.opcode & 0x0FFF);
}

fn op_Cxkk(cpu: &mut Cpu) {
    let x = ((cpu.opcode & 0x0F00) >> 8) as usize;
    let byte = cpu.opcode & 0x00FF;
    let mut rng = rand::thread_rng();
    let values = Uniform::from(1..256);

    cpu.V[x] = (values.sample(&mut rng) & byte) as u8;
}

fn op_Dxyn(cpu: &mut Cpu) {
    let x = ((cpu.opcode & 0x0F00) >> 8) as usize;
    let y = ((cpu.opcode & 0x0F00) >> 4) as usize;
    let height = cpu.opcode & 0x000F;

    let xPos = cpu.V[x] % WIDTH as u8; 
    let yPos = cpu.V[y] % HEIGHT as u8;

    cpu.V[0xF] = 0;

    for row in 0u16..height as u16{
        let spriteByte = cpu.memory[(cpu.I + row) as usize];
        
        for col in 0..8 {
            let spritePixel = spriteByte & (0x80 >> col);
            let mut screenPixel = cpu.gfx[((yPos + row as u8) * WIDTH as u8 + (xPos + col)) as usize];

            if spritePixel == 1 {
                if screenPixel == 0xFF {
                    cpu.V[0xF] = 1;
                }

                screenPixel ^= 0xFF;
            }
        }
    }
}

fn op_Ex9E(cpu: &mut Cpu) {
    let x = ((cpu.opcode & 0x0F00) >> 8) as usize;
    let key = cpu.V[x] as usize;

    if cpu.keypad[key] == 1 {
        cpu.pc += 2;
    }
}

fn op_ExA1(cpu: &mut Cpu) {
    let x = ((cpu.opcode & 0x0F00) >> 8) as usize;
    let key = cpu.V[x] as usize;

    if cpu.keypad[key] == 0 {
        cpu.pc += 2;
    }
}

fn op_Fx07(cpu: &mut Cpu) {
    let x = ((cpu.opcode & 0x0F00) >> 8) as usize;

    cpu.V[x] = cpu.delay_timer;
}

fn op_Fx0A(cpu: &mut Cpu) {
    let x = ((cpu.opcode & 0x0F00) >> 8) as usize;

    cpu.pc -= 2;

    for i in 0..16 {
        if cpu.keypad[i as usize] == 1 {
            cpu.V[x] = i;
            cpu.pc += 2;
        }
    }
}

fn op_Fx15(cpu: &mut Cpu) {
    let x = ((cpu.opcode & 0x0F00) >> 8) as usize;
    
    cpu.delay_timer = cpu.V[x];
}

fn op_Fx18(cpu: &mut Cpu) {
    let x = ((cpu.opcode & 0x0F00) >> 8) as usize;

    cpu.sound_timer = cpu.V[x];
}

fn op_Fx1E(cpu: &mut Cpu) {
    let x = ((cpu.opcode & 0x0F00) >> 8) as usize;

    cpu.I = cpu.V[x] as u16;
}

fn op_Fx29(cpu: &mut Cpu) {
    let x = ((cpu.opcode & 0x0F00) >> 8) as usize;
    
    cpu.I = 0x50 + (5 * cpu.V[x]) as u16;
}

fn op_Fx33(cpu: &mut Cpu) {
    let x = ((cpu.opcode & 0x0F00) >> 8) as usize;

    cpu.memory[cpu.I as usize + 2] = cpu.V[x] % 10;
    cpu.V[x] /= 10;

    cpu.memory[cpu.I as usize + 1] = cpu.V[x] % 10;
    cpu.V[x] /= 10;

    cpu.memory[cpu.I as usize] = cpu.V[x] % 10;
}

fn op_Fx55(cpu: &mut Cpu) {
    let x = ((cpu.opcode & 0x0F00) >> 8) as usize;

    for i in 0..(x + 1) {
        cpu.memory[cpu.I as usize + i] = cpu.V[i];
    }
}

fn op_Fx65(cpu: &mut Cpu) {
    let x = ((cpu.opcode & 0x0F00) >> 8) as usize;

    for i in 0..(x + 1) {
        cpu.V[i] = cpu.memory[cpu.I as usize + i];
    }
}


