use std::env;

mod lsm_asm;
mod lsm;

use device::ConsoleDevice;
mod device;

use lsm::LSM;
use lsm_asm::*;



fn main() {
    let mut console = ConsoleDevice::new(0, 4096);
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
        console.update(&mut lsm);
    

    }
    println!("");
    println!("~~> Terminated <~~");
}
