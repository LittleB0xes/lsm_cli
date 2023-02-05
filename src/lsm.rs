
use std::usize;

const MEMORY_SIZE: usize = 65536;
const STACK_SIZE: usize = 256;

pub struct LSM {
    pub memory: Vec<u8>,
        pc: usize,
    pub stack: Vec<u8>,
    pub system_call: [u8; 8],
    pub is_running: bool,
}

impl LSM {
    pub fn new() -> LSM {
        LSM {
            memory: vec![0; MEMORY_SIZE],
            pc: 0,
            stack: Vec::new(),
            system_call: [0; 8],
            is_running: true,

        }
    }
    pub fn load(&mut self, data: Vec<u8>) {
        for b in data.iter().enumerate() {
            self.memory[b.0] = *b.1;
        }
    }
    pub fn run(&mut self, step: bool) {

        // break if not is_running
        if !self.is_running {
            return;
        }

        while self.pc != self.memory.len() {
            match self.memory[self.pc] {
                0x00 => {self.is_running = false;break;},
                0x01 => {inc(&mut self.stack);},
                0x02 => {pop(&mut self.stack);},
                0x03 => {nip(&mut self.stack);},
                0x04 => {swp(&mut self.stack);},
                0x05 => {rot(&mut self.stack);},
                0x06 => {dup(&mut self.stack);},
                0x07 => {ovr(&mut self.stack);},
                0x08 => {equ(&mut self.stack);},
                0x09 => {neq(&mut self.stack);},
                0x0A => {gth(&mut self.stack);},
                0x0B => {lth(&mut self.stack);},
                0x0C => {
                    self.pc = jpu(&mut self.stack, self.pc);
                    self.pc -= 1;   // To balance the global pc += 1 below
                },
                0x0D => {
                    self.pc = jpd(&mut self.stack, self.pc);
                    self.pc -= 1;   // To balance the global pc += 1 below
                },
                0x0E => {
                    self.pc = jcn(&mut self.stack, self.pc);
                    self.pc -= 1;
                },
                0x0F => {
                    self.pc = jpr(&mut self.stack, self.pc);
                    self.pc -= 1;
                },
                0x14 => {lda(&mut self.stack, &mut self.memory);},
                0x15 => {sta(&mut self.stack, &mut self.memory);},
                0x18 => {add(&mut self.stack);}
                0x20 => {modulo(&mut self.stack);},
                0x21 => {div(&mut self.stack);},
                // Syscall
                // 0 for update_screen
                0x30 => {sys(&mut self.stack, &mut self.system_call);},
                0x80 => {
                    push(&mut self.stack, self.memory[self.pc + 1]);
                    self.pc += 1;
                },

                0xA0 => {
                    push(&mut self.stack, self.memory[self.pc + 1]);
                    push(&mut self.stack, self.memory[self.pc + 2]);
                    self.pc += 2;
                },
                _ => {}
            }
            self.pc += 1;

             println!("Stack: {:?}", self.stack);
            //println!("{} - {}", self.memory[1], self.memory[2]);

            if step {break;}
        }
    }
}


fn sys(s: &mut  Vec<u8>, sys_list: &mut [u8; 8]) {
    let call_id = pop(s);
    let value = pop(s);
    sys_list[call_id as usize] = value;
}

fn push(s: &mut Vec<u8>, v: u8) {
    s.push(v);
}

// INC a b c -- a b c+1
fn inc(s: &mut Vec<u8>) {
    let temp = pop(s); 
    push(s, temp + 1)
}

// POP a b c -- a b  -> c
fn pop(s: &mut Vec<u8>) -> u8 {
    s.pop().expect("Stack underflow")
}

// NIP a b c - a c  -> b
fn nip(s: &mut Vec<u8>) -> u8 {
    let t = pop(s);
    let r = pop(s);
    push(s, t);
    r
}


// SWP a b c -- a c b
fn swp(s: &mut Vec<u8>) {
    let t = nip(s);
    push(s, t);
}


// ROT a b c -- b c a
fn rot(s: &mut Vec<u8>) {
    let c = pop(s);
    let b = nip(s);
    push(s, c);
    push(s, b);
}


// DUP a b c -- a b c c
fn dup(s: &mut Vec<u8>) {
    let t = pop(s);
    push(s, t);
    push(s, t);
}

// OVR a b -- a b a
fn ovr(s: &mut Vec<u8>) {
    let b = pop(s);
    let a = pop(s);
    push(s, a); push(s, b); push(s, a);
}

// LDA a16 -- mem[a16] Load specific adress value in stack
fn lda(s: &mut Vec<u8>, m: &Vec<u8>) {
    let al = pop(s);
    let ah = pop(s);
    push(s, m[al as usize + (ah as usize) << 8]);
    
}


// STA a16 b -- store b in adress a
fn sta(s: &mut Vec<u8>, m: &mut Vec<u8>) {
    // Store value at adresse
    let al = pop(s) as usize;
    let ah = (pop(s) as usize) << 8;

    m[al + ah] = pop(s);
}

// EQU 
fn equ(s: &mut Vec<u8>) {
    if pop(s) == pop(s) {push(s,1)} else {push(s, 0)};
}

// ADD  a b -- a + b
fn add(s: &mut Vec<u8>) {
    let a = pop(s);
    let b = pop(s);
    push(s, a + b);
    
}

//NEQ 
fn neq(s: &mut Vec<u8>) {
    if pop(s) != pop(s) {push(s,1)} else {push(s, 0)};
}

// GTH a b -- a > b(greater than)
fn gth(s: &mut Vec<u8>) {
    if pop(s) < pop(s) {push(s, 1)} else { push(s,0)}
}


/// LTH a b -- a < b Less than
fn lth(s: &mut Vec<u8>) {
    if pop(s) > pop(s) { push(s, 1)} else { push(s,0) }
}


/// JPU a --  Jump to the relative upper adress
fn jpu(s: &mut Vec<u8>, pc: usize) -> usize {
    pc + pop(s) as usize
}
/// JPL a --  Jump to the relative Lower adress
fn jpd(s: &mut Vec<u8>, pc: usize) -> usize {
    pc - pop(s) as usize
}
/// JPR (Jump to th relative adress) rel_adress -- 
fn jpr(s: &mut Vec<u8>, pc: usize) -> usize {
    (pc as isize + (pop(s) as i8) as isize) as usize
}


/// JCN (Conditional jump) cond rel_adress --
fn jcn(s: &mut Vec<u8>, pc: usize) -> usize {
    if nip(s) == 1 {
        jpr(s, pc)
    }
    else {
        pop(s);  // Remove the adress
        pc + 1
    }
}

/// MOD a b -- a % b
fn modulo(s: &mut Vec<u8>) {
    let b = pop(s);
    let a = pop(s);
    push(s, a%b);
}

/// DIV a b -- a/b
fn div(s: &mut Vec<u8>) {
    let b = pop(s);
    let a = pop(s);
    push(s, a/b);
}


#[cfg(test)]
mod test_opcode {
    use crate::lsm::*;

    fn bool_to_int(value: bool) -> u8 {
        if value {
            1
        }
        else {
            0
        }
    }

    
    #[test]
    fn add_test() {
        for set in vec![vec![12,33],vec![39, 0],vec![0, 24],vec![0, 0]].iter_mut() {
            let a = set[0];
            let b = set[1];
            add(set);
            assert_eq!(set[0], a + b);
        }
    }

    #[test]
    fn ovr_test() {
        let mut input = vec![1, 2, 3];
        ovr(&mut input);
        assert_eq!(input, vec![1,2,3,2]);
    }

    #[test]
    fn equ_test() {
        for s in [(vec![2,2], [1]), (vec![2,3], [0]) , (vec![3,2], [0])].iter_mut() {
            equ(&mut s.0);
            assert_eq!(s.0, s.1 );
        }
    }
    #[test]
     fn lth_test() {
        for s in [vec![2,2], vec![2,3] , vec![3,2]].iter_mut() {
            let a = s[0];
            let b = s[1];
            lth(s);
            assert_eq!(s.pop().expect("Empty stack"), bool_to_int(b > a) );
        }
     }
    
    #[test]
    fn gth_test() {
        for s in [vec![2,2], vec![2,3] , vec![3,2]].iter_mut() {
            let a = s[0];
            let b = s[1];
            gth(s);
            assert_eq!(s.pop().expect("Empty stack"), bool_to_int(b < a) );
        }
    }
    #[test]
    fn neq_test() {
        for s in [(vec![2,2], [0]), (vec![2,3], [1]) , (vec![3,2], [1])].iter_mut() {
            neq(&mut s.0);
            assert_eq!(s.0, s.1 );
        }
    }

    #[test]
    fn jpr_test() {
        // Test data (relative adress, current pc, destination pc)
        for set in [(2, 10, 12), (-2, 10, 8 ) , (0, 10, 10), (-2, 255, 253)]{
            let mut s: Vec<u8> = vec!(set.0 as u8);
            let result = set.2;
            let new_adress = jpr(&mut s, set.1);

            // check empty stack
            // println!("Stack checking {:?} ---", set);
            assert_eq!(s.len(), 0 );

            // check pc translation
            // println!("pc checking {:?} ---", set);
            assert_eq!(new_adress, result);
        }
    }
}
