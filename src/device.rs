use crate::lsm::LSM;


enum ConsoleFlags {
    Add = 0x01,
    Refresh = 0x02,
}



pub struct ConsoleDevice {
    pub id: u8,
    pub adress: usize,
    cache: Vec<u8>,
    refresh: bool,
}

impl ConsoleDevice {
    pub fn new(id: u8, adress: usize) -> ConsoleDevice {
        ConsoleDevice {
            id,
            adress,
            cache: Vec::new(),
            refresh: false,
        }
    }

    pub fn update(&mut self, lsm: &mut LSM) {
        let character = lsm.memory[self.adress];
        let mut flags = lsm.system_call[self.id as usize]; 
        if flags  & ConsoleFlags::Add as u8 != 0 {
            flags = flags ^ ConsoleFlags::Add as u8;
            self.cache.push(character as u8);
        }

        self.refresh =  (flags & ConsoleFlags::Refresh as u8) != 0;
        
        if self.refresh {
            println!("Refresh");
            for c in self.cache.iter() {
                print!("{}", *c as char);
            }
            flags = flags ^ ConsoleFlags::Refresh as u8;
            self.cache = Vec::new();
        }
        lsm.system_call[self.id as usize] = flags;
        
    }

}

