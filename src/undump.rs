use anyhow::Result;
use core::panic;
use std::io::{Cursor, Read, Seek, SeekFrom};

#[derive(Debug, PartialEq)]
#[repr(u8)]
enum Endian {
    BigEndian = 0u8,
    LittleEndian = 1u8,
}

#[derive(Debug, PartialEq)]
#[repr(u8)]
enum Integral {
    FloatingPoint = 0u8,
    IntegralNumber = 1u8,
}

#[derive(Debug, PartialEq)]
#[repr(C)]
struct LuaHeader {
    signature: [u8; 4], // 0x1b4c7561
    lua_version: u8,    // 0x51
    format_version: u8, // 0x00
    endian: Endian,     // 0x00=big endian, 0x01=little endian
    int_size: u8,       // default 4
    size_t_size: u8,    // default 4
    inst_size: u8,      // default 4
    number_size: u8,    // default 8
    integral: Integral, // 0x00=floating-point, 0x01=integral number type default 0
}


#[derive(Debug, PartialEq)]
struct Undump {
    cur: Cursor<Vec<u8>>,
}

impl Undump {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self {
            cur: Cursor::new(bytes),
        }
    }
    fn read_header(&mut self) -> Result<LuaHeader> {
        let mut signature = [0u8; 4];
        let mut lua_version = [0u8; 1];
        let mut format_version = [0u8; 1];
        let mut endian = [0u8; 1];
        let mut int_size = [0u8; 1];
        let mut size_t_size = [0u8; 1];
        let mut inst_size = [0u8; 1];
        let mut number_size = [0u8; 1];
        let mut integral = [0u8; 1];
        self.cur.read_exact(&mut signature)?;
        self.cur.read_exact(&mut lua_version)?;
        self.cur.read_exact(&mut format_version)?;
        self.cur.read_exact(&mut endian)?;
        self.cur.read_exact(&mut int_size)?;
        self.cur.read_exact(&mut size_t_size)?;
        self.cur.read_exact(&mut inst_size)?;
        self.cur.read_exact(&mut number_size)?;
        self.cur.read_exact(&mut integral)?;
        Ok(LuaHeader {
            signature,
            lua_version: u8::from_be_bytes(lua_version),
            format_version: u8::from_be_bytes(format_version),
            endian: match u8::from_be_bytes(endian) {
                0u8 => Endian::BigEndian,
                1u8 => Endian::LittleEndian,
                _ => panic!("Invalid Endian"),
            },
            int_size: u8::from_be_bytes(int_size),
            size_t_size: u8::from_be_bytes(size_t_size),
            inst_size: u8::from_be_bytes(inst_size),
            number_size: u8::from_be_bytes(number_size),
            integral: match u8::from_be_bytes(integral) {
                0u8 => Integral::FloatingPoint,
                1u8 => Integral::IntegralNumber,
                _ => panic!("Invalid Integral"),
            },
        })
    }
    fn read_function(&mut self) {}
    pub fn undump(&self) {}

    pub fn print(&self) {}
}

#[cfg(test)]
mod tests {
    use crate::undump::{Endian, Integral, LuaHeader, Undump};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_header() {
        let mut bytecodes = Vec::<u8>::new();
        bytecodes.extend(&0x1b4c7561u32.to_be_bytes()); // signature
        bytecodes.extend(&0x51u8.to_be_bytes()); // lua_version
        bytecodes.extend(&0x00u8.to_be_bytes()); // format_version
        bytecodes.extend(&0x01u8.to_be_bytes()); // endian
        bytecodes.extend(&0x04u8.to_be_bytes()); // int_size
        bytecodes.extend(&0x04u8.to_be_bytes()); // size_t_size
        bytecodes.extend(&0x04u8.to_be_bytes()); // inst_size
        bytecodes.extend(&0x08u8.to_be_bytes()); // number_size
        bytecodes.extend(&0x00u8.to_be_bytes()); // integral
        let result = bytecodes
            .iter()
            .map(|n| format!("{:02X}", n))
            .collect::<String>();
        println!("{}", result);
        let mut undump = Undump::new(bytecodes);
        assert_eq!(
            undump.read_header().unwrap(),
            LuaHeader {
                signature: 0x1b4c7561u32.to_be_bytes(),
                lua_version: 0x51u8,
                format_version: 0x00u8,
                endian: Endian::LittleEndian,
                int_size: 0x04u8,
                size_t_size: 0x04u8,
                inst_size: 0x04u8,
                number_size: 0x08u8,
                integral: Integral::FloatingPoint,
            }
        );
        assert_eq!(12, std::mem::size_of::<LuaHeader>());
    }
}
