// 解析二进制chunk的reader模块

// 定义模块
pub mod reader;

#[derive(Debug)]
pub struct Reader {
    data: Vec<u8>,
    pos: usize,
}

impl Reader {
    pub fn new(data: Vec<u8>) -> Self {
        Reader { data, pos: 0 }
    }

    // 读取一个字节
    pub fn read_byte(&mut self) -> u8 {
        let byte = self.data[self.pos];
        self.pos += 1;
        byte
    }

    // 读取一个u32
    pub fn read_u32(&mut self) -> u32 {
        let mut result = 0;
        for i in 0..4 {
            result |= (self.read_byte() as u32) << (i * 8);
        }
        result
    }

    // 读取一个u64
    pub fn read_u64(&mut self) -> u64 {
        let mut result = 0;
        for i in 0..8 {
            result |= (self.read_byte() as u64) << (i * 8);
        }
        result
    }

    // 读取一个f64
    pub fn read_f64(&mut self) -> f64 {
        let mut result = 0.0;
        for i in 0..8 {
            result += (self.read_byte() as f64) * 2f64.powi((i * 8) as i32);
        }
        result
    }

    // 读取一个string
    pub fn read_string(&mut self) -> String {
        let mut bytes = vec![];
        loop {
            let byte = self.read_byte();
            if byte == 0 {
                break;
            }
            bytes.push(byte);
        }
        String::from_utf8(bytes).unwrap()
    }

    // 读取多个字节
    pub fn read_bytes(&mut self, n: usize) -> Vec<u8> {
        let mut bytes = vec![];
        for _ in 0..n {
            bytes.push(self.read_byte());
        }
        bytes
    }

    // 检查头部
    pub fn check_header(&mut self) {
        let header: [u8; 4] = [0x1b, b'L', b'u', b'a'];
        for i in 0..4 {
            assert_eq!(self.read_byte(), header[i]);
        }
        assert_eq!(self.read_byte(), 0x53);
        assert_eq!(self.read_byte(), 0);
        assert_eq!(self.read_bytes(6), [0x19, 0x93, 0x0d, 0x0a, 0x1a, 0x0a]);
        assert_eq!(self.read_byte(), 4);
        assert_eq!(self.read_byte(), 8);
        assert_eq!(self.read_byte(), 4);
        assert_eq!(self.read_byte(), 8);
        assert_eq!(self.read_byte(), 8);
        assert_eq!(self.read_byte(), 8);
        assert_eq!(self.read_f64(), 370.5);
        assert_eq!(self.read_u64(), 0x5678);
    }

    // 读取原型
    pub fn read_proto(&mut self, parent_source: String) -> Prototype {
        let source = self.read_string();
        if source.is_empty() {
            Prototype::new()
        } else {
            let line_defined = self.read_u32();
            let last_line_defined = self.read_u32();
            let num_params = self.read_byte();
            let is_vararg = self.read_byte();
            let max_stack_size = self.read_byte();
            let code = self.read_code();
            let constants = self.read_constants();
            let mut reader = Reader::new(self.read_bytes(4));
            let size_up_values = reader.read_u32();
            let up_values = self.read_up_values(size_up_values as usize);
            let size_protos = self.read_u32();
            let protos = self.read_protos(size_protos as usize, source.clone());
            let line_info = self.read_line_info();
            let loc_vars = self.read_loc_vars();
            let up_value_names = self.read_up_value_names();
            Prototype {
                source,
                line_defined,
                last_line_defined,
                num_params,
                is_vararg,
                max_stack_size,
                code,
                constants,
                up_values,
                protos,
                line_info,
                loc_vars,
                up_value_names,
            }
        }
    }

    // 读取指令表
    pub fn read_code(&mut self) -> Vec<u32> {
        let size = self.read_u32();
        let mut code = vec![];
        for _ in 0..size {
            code.push(self.read_u32());
        }
        code
    }

    // 读取常量表
    pub fn read_constants(&mut self) -> Vec<Constant> {
        let size = self.read_u32();
        let mut constants = vec![];
        for _ in 0..size {
            let tag = self.read_byte();
            match tag {
                0 => {
                    constants.push(Constant::Nil);
                }
                1 => {
                    constants.push(Constant::Boolean(self.read_byte() != 0));
                }
                _ => {
                    constants.push(Constant::Number(self.read_f64()));
                }
            }
        }
        constants
    }

    // 读取Up value表
    pub fn read_up_values(&mut self, n: usize) -> Vec<UpValue> {
        let mut up_values = vec![];
        for _ in 0..n {
            up_values.push(UpValue {
                in_stack: self.read_byte(),
                idx: self.read_byte(),
            });
        }
        up_values
    }

    // 读取原型表
    pub fn read_protos(&mut self, n: usize, parent_source: String) -> Vec<Prototype> {
        let mut protos = vec![];
        for _ in 0..n {
            protos.push(self.read_proto(parent_source.clone()));
        }
        protos
    }

    // 读取行号表
    pub fn read_line_info(&mut self) -> Vec<u32> {
        let size = self.read_u32();
        let mut line_info = vec![];
        for _ in 0..size {
            line_info.push(self.read_u32());
        }
        line_info
    }

    // 读取局部变量表
    pub fn read_loc_vars(&mut self) -> Vec<LocVar> {
        let size = self.read_u32();
        let mut loc_vars = vec![];
        for _ in 0..size {
            loc_vars.push(LocVar {
                var_name: self.read_string(),
                start_pc: self.read_u32(),
                end_pc: self.read_u32(),
            });
        }
        loc_vars
    }

    // 读取Up value名列表
    pub fn read_up_value_names(&mut self) -> Vec<String> {
        let size = self.read_u32();
        let mut up_value_names = vec![];
        for _ in 0..size {
            up_value_names.push(self.read_string());
        }
        up_value_names
    }
}
