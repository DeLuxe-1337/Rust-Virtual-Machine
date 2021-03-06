mod vm;

use vm::VM;
use std::io::Read;

use std::time::{Duration, Instant};

fn main() {
    let now = Instant::now();
    let mut c = VM::new();

    let mut file = std::fs::File::open("program.bin").expect("Expected file");

    let mut program = [0; 4096];

    file.read(&mut program).expect("Expected read");

    let converted = program.iter().map(|&x| x as i32).collect::<Vec<i32>>();

    c.load(converted.as_slice());

    c.run();
    //c.dump(25, 5);

    println!("It took {} ms to run!", now.elapsed().as_millis());

    std::io::stdin().read(&mut [0]).unwrap();
}

/*

!--Registers--!

? You can use up to 1000 registers.

!-------------!

* 0x1 = nop
* 0x2 = push num, register
* 0x3 = jmp num
* 0x4 = jmp if reg = num (jmp to, reg, num)

* 0x5 = add reg,reg
* 0x6 = sub
* 0x7 = mul
* 0x8 = div

* 0x9 = function constant

* 0x10 = print mode, reg
* 0x11 = function_call constant
* 0x15 = constant

? Print_Modes 
    0x0 = print reg
    0x1 = print constant

*/