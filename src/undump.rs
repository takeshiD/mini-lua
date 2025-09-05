use anyhow::Result;
use core::panic;
use std::{
    io::{Cursor, Read},
    vec,
};

#[derive(Debug, PartialEq)]
struct Local {
    name: String,
    start_line: LuaInt,
    end_line: LuaInt,
}

impl Local {
    pub fn new(name: String, start_line: LuaInt, end_line: LuaInt) -> Self {
        Self {
            name,
            start_line,
            end_line,
        }
    }
}

#[derive(Debug, PartialEq)]
struct MetaInfo {
    first_line: LuaInt,
    last_line: LuaInt,
    num_upvals: LuaInt,
    num_params: u8,
    is_varg: bool,
    max_stack: u8,
}

#[derive(Debug, PartialEq)]
pub struct Chunk {
    name: String,
    meta_info: MetaInfo,
    instructions: Vec<u32>,
    constant_table: Vec<Constant>,
    protos: Vec<Chunk>,
    lines: Vec<LuaInt>,
    locals: Vec<Local>,
    upvalues: Vec<String>,
}

#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum Constant {
    Nil,
    Bool(bool),
    Number(f64),
    String(String),
}

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
struct Header {
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

enum SizeT {
    U0,
    U32(u32),
    U64(u64),
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
enum LuaInt {
    U32(u32),
    U64(u64),
}

impl From<LuaInt> for usize {
    fn from(item: LuaInt) -> usize {
        match item {
            LuaInt::U32(n) => n as usize,
            LuaInt::U64(n) => n as usize,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Undump {
    cur: Cursor<Vec<u8>>,
}

impl Undump {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self {
            cur: Cursor::new(bytes),
        }
    }
    fn read_header(&mut self) -> Result<Header> {
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
        Ok(Header {
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

    fn read_float32(&mut self, header: &Header) -> Result<f32> {
        let mut buf = [0u8; 4];
        self.cur.read_exact(&mut buf)?;
        match header.endian {
            Endian::BigEndian => Ok(f32::from_be_bytes(buf)),
            Endian::LittleEndian => Ok(f32::from_le_bytes(buf)),
        }
    }

    fn read_float64(&mut self, header: &Header) -> Result<f64> {
        let mut buf = [0u8; 8];
        self.cur.read_exact(&mut buf)?;
        match header.endian {
            Endian::BigEndian => Ok(f64::from_be_bytes(buf)),
            Endian::LittleEndian => Ok(f64::from_le_bytes(buf)),
        }
    }

    fn read_uint32(&mut self, header: &Header) -> Result<u32> {
        let mut buf = [0u8; 4];
        self.cur.read_exact(&mut buf)?;
        match header.endian {
            Endian::BigEndian => Ok(u32::from_be_bytes(buf)),
            Endian::LittleEndian => Ok(u32::from_le_bytes(buf)),
        }
    }

    fn read_uint64(&mut self, header: &Header) -> Result<u64> {
        let mut buf = [0u8; 8];
        self.cur.read_exact(&mut buf)?;
        match header.endian {
            Endian::BigEndian => Ok(u64::from_be_bytes(buf)),
            Endian::LittleEndian => Ok(u64::from_le_bytes(buf)),
        }
    }

    fn read_size_t(&mut self, header: &Header) -> Result<SizeT> {
        match header.size_t_size {
            4 => {
                let mut buf = [0u8; 4];
                self.cur.read_exact(&mut buf)?;
                match header.endian {
                    Endian::BigEndian => Ok(SizeT::U32(u32::from_be_bytes(buf))),
                    Endian::LittleEndian => Ok(SizeT::U32(u32::from_le_bytes(buf))),
                }
            }
            8 => {
                let mut buf = [0u8; 8];
                self.cur.read_exact(&mut buf)?;
                match header.endian {
                    Endian::BigEndian => Ok(SizeT::U64(u64::from_be_bytes(buf))),
                    Endian::LittleEndian => Ok(SizeT::U64(u64::from_le_bytes(buf))),
                }
            }
            n => {
                unimplemented!("any size_t is not implemeted... got '{n}'")
            }
        }
    }

    fn read_string(&mut self, header: &Header) -> Result<String> {
        match self.read_size_t(header)? {
            SizeT::U0 => Ok("".to_string()),
            SizeT::U32(size) => {
                let mut string_bytes = vec![0u8; size as usize];
                self.cur.read_exact(&mut string_bytes)?;
                let ret = String::from_utf8(string_bytes)?;
                Ok(ret)
            }
            SizeT::U64(size) => {
                let mut string_bytes = vec![0u8; size as usize];
                self.cur.read_exact(&mut string_bytes)?;
                let ret = String::from_utf8(string_bytes)?;
                Ok(ret)
            }
        }
    }

    fn read_uint(&mut self, header: &Header) -> Result<LuaInt> {
        match header.int_size {
            4 => {
                let mut buf = [0u8; 4];
                self.cur.read_exact(&mut buf)?;
                match header.endian {
                    Endian::BigEndian => Ok(LuaInt::U32(u32::from_be_bytes(buf))),
                    Endian::LittleEndian => Ok(LuaInt::U32(u32::from_le_bytes(buf))),
                }
            }
            8 => {
                let mut buf = [0u8; 8];
                self.cur.read_exact(&mut buf)?;
                match header.endian {
                    Endian::BigEndian => Ok(LuaInt::U64(u64::from_be_bytes(buf))),
                    Endian::LittleEndian => Ok(LuaInt::U64(u64::from_le_bytes(buf))),
                }
            }
            n => unimplemented!("any int size is not implemeted... got '{n}'"),
        }
    }

    fn read_byte(&mut self) -> Result<u8> {
        let mut buf = [0u8; 1];
        self.cur.read_exact(&mut buf)?;
        Ok(u8::from_le_bytes(buf))
    }

    fn read_chunk(&mut self, header: &Header) -> Result<Chunk> {
        // meta info
        let name = self.read_string(header)?;
        let first_line = self.read_uint(header)?;
        let last_line = self.read_uint(header)?;
        let num_upval = self.read_byte()?;
        let num_params = self.read_byte()?;
        let is_varg = self.read_byte()? != 0u8;
        let max_stack = self.read_byte()?;

        // instructions
        let num_insts = self.read_uint(header)?;
        let mut insts = vec![];
        for _ in 0..usize::from(num_insts) {
            insts.push(self.read_uint32(header)?);
        }
        // constant table
        let num_consts = self.read_uint(header)?;
        let mut consts = vec![];
        for _ in 0..usize::from(num_consts) {
            match self.read_byte()? {
                0 => consts.push(Constant::Nil),
                1 => consts.push(Constant::Bool(self.read_byte()? != 0)),
                3 => consts.push(Constant::Number(self.read_float64(header)?)),
                4 => consts.push(Constant::String(self.read_string(header)?)),
                n => panic!("unknown datatype: actually got '{n}'"),
            }
        }
        // proto
        let num_protos = self.read_uint(header)?;
        let mut protos = vec![];
        for _ in 0..usize::from(num_protos) {
            protos.push(self.read_chunk(header)?);
        }

        // number_of_lines
        let num_lines = self.read_uint(header)?;
        let mut lines = vec![];
        for _ in 0..usize::from(num_lines) {
            lines.push(self.read_uint(header)?);
        }

        // local list
        let num_locals = self.read_uint(header)?;
        let mut locals = vec![];
        for _ in 0..usize::from(num_locals) {
            let name = self.read_string(header)?;
            let start_line = self.read_uint(header)?;
            let end_line = self.read_uint(header)?;
            locals.push(Local::new(name, start_line, end_line));
        }

        // upvalue
        let num_upvals = self.read_uint(header)?;
        let mut upvals = vec![];
        for _ in 0..usize::from(num_upvals) {
            let name = self.read_string(header)?;
            upvals.push(name);
        }
        Ok(Chunk {
            name,
            meta_info: MetaInfo {
                first_line,
                last_line,
                num_upvals,
                num_params,
                is_varg,
                max_stack,
            },
            instructions: insts,
            constant_table: consts,
            protos,
            lines,
            locals,
            upvalues: upvals,
        })
    }
    pub fn undump(&mut self) -> Result<(Header, Chunk)> {
        let header = self.read_header()?;
        let chunk = self.read_chunk(&header)?;
        Ok((header, chunk))
    }

    pub fn print(&mut self) {
        match self.undump() {
            Ok((h, c)) => {
                println!("header: {:#?}", h);
                println!("chunk: {:#?}", c);
            }
            Err(e) => {
                eprintln!("{e}");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::undump::{Endian, Header, Integral, Undump};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_header() {
        let mut bytecodes = Vec::<u8>::new();
        // Bytes: 1b4c_7561_5100_0104_0404_0800
        bytecodes.extend(&0x1b4c7561u32.to_be_bytes()); // signature
        bytecodes.extend(&0x51u8.to_be_bytes()); // lua_version
        bytecodes.extend(&0x00u8.to_be_bytes()); // format_version
        bytecodes.extend(&0x01u8.to_be_bytes()); // endian
        bytecodes.extend(&0x04u8.to_be_bytes()); // int_size
        bytecodes.extend(&0x04u8.to_be_bytes()); // size_t_size
        bytecodes.extend(&0x04u8.to_be_bytes()); // inst_size
        bytecodes.extend(&0x08u8.to_be_bytes()); // number_size
        bytecodes.extend(&0x00u8.to_be_bytes()); // integral
        assert_eq!(true, false);
        let result = bytecodes
            .iter()
            .map(|n| format!("{:02x}", n))
            .collect::<String>();
        println!("{}", result);
        let mut undump = Undump::new(bytecodes);
        assert_eq!(
            undump.read_header().unwrap(),
            Header {
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
        assert_eq!(12, std::mem::size_of::<Header>());
    }
}
