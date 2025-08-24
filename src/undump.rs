struct LuaHeader {
    version: u8,
    format_version: u8,
    big_endian: bool,
}

struct Undump {
    bytecodes: [u8],
}

impl Undump {
    pub fn new(bytecodes: &[u8]) -> Self {
        Self {
            bytecodes
        }
    }
    fn read_header(&self) {

    }
    fn read_function(&self) {

    }
    pub fn undump(&self) {

    }

    pub fn print(&self) {

    }
}
