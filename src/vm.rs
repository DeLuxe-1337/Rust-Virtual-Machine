pub struct VM {
    pub memory: [i32; 4096],
    pub registers: [i32; 1000],
    pub constants: Vec<String>,
    pub pc: usize
}

impl VM { 
    pub fn new() -> VM {
        return VM {
            memory: [0; 4096],
            registers: [0; 1000],
            constants: Vec::new(),
            pc: 0,
        };
    }
    pub fn load(&mut self, program: &[i32]) {
        for (i, &byte) in program.iter().enumerate() {
            self.memory[i] = byte;
        }
    }
    pub fn new_constant(&mut self, name: &str) {
        println!("New constant: {}", name);
        self.constants.push(name.to_string());
    }
    pub fn dump(&self, to: usize, reg: usize) {
        println!("--Debug----------------------------------------------------");
        println!("--Registers--");

        for (i, b) in self.registers.iter().enumerate() {
            if i == reg {
                println!(";");
                break;
            }

            print!("Register {0} = {1} ", i, b);
        }

        println!("--Memory--");

        for (i, b) in self.memory.iter().enumerate() {
            if i == to {
                println!(";");
                break;
            }

            print!("{} ", b);
        }

        println!("-----------------------------------------------------------");
    }
    fn step(&mut self) {
        self.pc += 1;
    }
    fn next(&mut self) -> i32 {
        self.step();
        return self.memory[self.pc];
    }
    fn setr(&mut self, register: usize, value: i32) {
        self.registers[register] = value;
    }
    pub fn run(&mut self) {
        while self.pc < self.memory.len() {
            let op = self.memory[self.pc];

            //println!("PC: 0x{:x}", self.pc);

            match op {
                0x1 | 0x0 => {
                    self.step();
                },
                0x2 => {
                    let value = self.next();
                    let register = self.next() as usize;
                    
                    self.setr(register, value);

                    self.step();

                    println!("push {1}, reg({0})", register, value);
                },
                0x3 => {
                    let value = self.next();
                    self.pc = value as usize;

                    println!("jmp 0x{:x}", value);
                },
                0x4 => {
                    let jmp = self.next() as usize;
                    let reg = self.next() as usize;
                    let value = self.next();

                    if self.registers[reg] == value {
                        self.pc = jmp;
                    }

                    println!("jmpeq 0x{1:x} if reg({0}) == {2}", reg, jmp, value);
                },
                0x5 => {
                    let register_a = self.next() as usize;
                    let register_b = self.next() as usize;

                    self.setr(register_a, self.registers[register_a] + self.registers[register_b]);
                    self.setr(register_b, 0x0);

                    self.step();

                    println!("add reg({0}), reg({1})", register_a, register_b);
                },
                0x6 => {
                    let register_a = self.next() as usize;
                    let register_b = self.next() as usize;

                    self.setr(register_a, self.registers[register_a] - self.registers[register_b]);
                    self.setr(register_b, 0x0);

                    self.step();

                    println!("sub reg({0}), reg({1})", register_a, register_b);
                },
                0x7 => {
                    let register_a = self.next() as usize;
                    let register_b = self.next() as usize;

                    self.setr(register_a, self.registers[register_a] * self.registers[register_b]);
                    self.setr(register_b, 0x0);

                    self.step();

                    println!("mul reg({0}), reg({1})", register_a, register_b);
                },
                0x8 => {
                    let register_a = self.next() as usize;
                    let register_b = self.next() as usize;

                    self.setr(register_a, self.registers[register_a] / self.registers[register_b]);
                    self.setr(register_b, 0x0);

                    self.step();

                    println!("div reg({0}), reg({1})", register_a, register_b);
                },
                0x10 => {
                    let mode = self.next();
                    let register = self.next() as usize;

                    match mode {
                        0x0 => {
                            println!("{}", self.registers[register]);
                        }
                        0x1 => {
                            println!("{}", self.constants[self.registers[register] as usize]);
                        }
                        _ => {}
                    }

                    println!("print mode({0}) {1}", mode, register);
                },
                0x15 => {
                    let mut storage: Vec<i32> = Vec::new();

                    while self.pc < self.memory.len() {
                        let op = self.next();

                        if op == 0x15 {
                            self.step();
                            break;
                        }

                        storage.push(op);
                    }

                    let constant: String = storage.iter().map(|&x| x as u8 as char).collect();

                    self.new_constant(constant.as_str());
                },
                _ => {
                    println!("Unknown opcode: 0x{:x}", op);
                    self.step();
                }
            }
        }
    }
}