// use std::env;
use std::fs;

pub fn assemble_file(path: &str ) -> Vec<u8>{
    let content: &str = &fs::read_to_string(path).expect("lsm file not found");
    
    assemble(content)
    
}
pub fn assemble(data: &str) -> Vec<u8> {
    let splited_data: Vec<&str> = parse_data(data);
    let mut code: Vec<u8> = Vec::new();

    // for instruction in splited_data.iter() {
    let mut index = 0;
    while index < splited_data.len() {
        let instruction = splited_data[index];
        match find_opcode(instruction) {
            Some(opcode) => match opcode {

                // If LIT
                0x80 => {
                        code.push(opcode);
                        code.push(u8::from_str_radix(&splited_data[index + 1], 16)
                                  .expect("Unexpected instruction"));
                        index += 1},
                // if LITd
                0xA0 => {
                        code.push(opcode);
                        code.push(u8::from_str_radix(&splited_data[index + 1], 16)
                                  .expect("Unexpected instruction"));
                        code.push(u8::from_str_radix(&splited_data[index + 2], 16)
                                  .expect("Unexpected instruction"));
                        index += 2},
                _ => {code.push(opcode)}
            },
            None => {
                if instruction.len() == 2 {
                    // Push le LIT opcode
                    code.push(0x80);

                    // Push the value
                    code.push(u8::from_str_radix(&instruction, 16)
                              .expect("Unexpected instruction"));

                }
                else if instruction.len() == 4 {
                    // Push le LIT opcode
                    code.push(0xA0);

                    // Push the value
                    code.push(u8::from_str_radix(&instruction[0..2], 16)
                              .expect("Unexpected instruction"));
                    code.push(u8::from_str_radix(&instruction[2..4], 16)
                              .expect("Unexpected instruction"));

                }


            }
        }
        index += 1;
    }

    code
}

/// Convert &str data to Vec<u8> without all unuseful space and CR
fn parse_data(data: &str) -> Vec<&str> {

    let mut splited: Vec<&str> = data.split(|c| c == ' ' || c == '\n').collect();
    splited.retain(|l| l != &"");
    splited
    
}

fn find_opcode(c: &str) -> Option<u8> {
    match c {
        "BRK" => Some(0x00),
        "INC" => Some(0x01),
        "POP" => Some(0x02),
        "NIP" => Some(0x03),
        "SWP" => Some(0x04),
        "ROT" => Some(0x05),
        "DUP" => Some(0x06),
        "OVR" => Some(0x07),
        "EQU" => Some(0x08),
        "NEQ" => Some(0x09),
        "GTH" => Some(0x0A),
        "LTH" => Some(0x0B),
        "JPU" => Some(0x0C),
        "JPL" => Some(0x0D),
        "JCN" => Some(0x0E),
        "JPR" => Some(0x0F),
        

        "DRW" => Some(0x13),
        "LDA" => Some(0x14),
        "STA" => Some(0x15),
        "ADD" => Some(0x18),

        "MOD" => Some(0x20),
        "DIV" => Some(0x21),

        "SYS" => Some(0x30),

        "LIT" => Some(0x80),
        "LITd" => Some(0xA0),

        _ => None
    }
}


#[cfg(test)]
mod test_main {
    use crate::lsm_asm::*;   
    #[test]
    fn parse_space_test() {
        let set = "00 00 01   STA   00 ";
        assert_eq!(vec!["00", "00", "01", "STA", "00"], parse_data(set));
    }
    #[test]
    fn parse_return_test() {
        let set = "\n00 00 01\nSTA 00\n";
        assert_eq!(vec!["00", "00", "01", "STA", "00"], parse_data(set));
    }

    #[test]
    fn assemble_lit_test() {
        let set = "LIT 01 00 STA";
        assert_eq!(assemble(set), vec![0x80, 0x01,0x80, 0x0, 0x15]);
    }

    #[test]
    fn assemble_litd_test() {
        // let set = "LITd 01 00 STA";
        let set = "LITd 10 20 03 JPU";
        assert_eq!(assemble(set), vec![0xa0, 0x10, 0x20, 0x80, 0x03, 0x0c]);
    }

}


