use std::{env, char};

mod lsm_asm;
mod lsm;

use lsm::LSM;
use lsm_asm::*;


struct OutputDevice {
    adress: u16,
    id: usize,
}

impl OutputDevice {
    fn update(&self, sys_list: &mut [u8; 8], memory: &Vec<u8> ) {
        match sys_list[self.id] {
            1 => self.print_char(memory),
            _ => {},
        }
        // device inactivation
        sys_list[self.id] = 0;
    }
    fn print_char(&self, memory: &Vec<u8>) {
        let mut index = self.adress as usize;
        while memory[index] != 0 {
            print!("{}", memory[index] as char);
            index += 1;
        }

    }

}


fn main() {
    let output = OutputDevice{adress: 4096, id: 0};
    println!("~~> Like A Stack Machine <~~");
    let args: Vec<String> = env::args().collect();
    
    // Take the code in argument
    let code = assemble_file(&args[1]);
    // println!("Code {}", code.iter().map(|c| format!(" {:#04x} |", c)).collect::<String>());

    let mut lsm = LSM::new();
    lsm.load(code);

    // Main loop
    while lsm.is_running {
        lsm.run(true);
        //output.update(&mut lsm.system_call, &mut lsm.memory);
    

    }
    println!("");
    println!("~~> Terminated <~~");
}
