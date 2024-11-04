#[cfg(test)]
#[derive(Debug)]
pub struct BinaryChunk {
    header: Header,
    size_up_values: u8,
    main_func: Prototype,
}

#[derive(Debug)]
pub struct Header {
    signature: [u8; 4], // 魔数
    version: u8,        // 版本号
    format: u8,         // 格式
    luac_data: [u8; 6], // 与平台无关
    c_int_size: u8,
    size_t_size: u8,
    instruction_size: u8,
    lua_integer_size: u8,
    lua_number_size: u8,
    luac_int: i64,
    luac_num: f64,
}

#[derive(Debug)]
pub struct Prototype {
    source: String,
    line_defined: u32,
    last_line_defined: u32,
    num_params: u8,
    is_vararg: u8,
    max_stack_size: u8,
    code: Vec<u32>,
    constants: Vec<Constant>,
    up_values: Vec<UpValue>,
    protos: Vec<Prototype>,
    line_info: Vec<u32>,
    loc_vars: Vec<LocVar>,
    up_value_names: Vec<String>,
}

#[derive(Debug)]
pub struct Constant {
    tag: u8,
    value: ConstantValue,
}

#[derive(Debug)]
pub enum ConstantValue {
    Nil,
    Boolean(bool),
    Integer(i64),
    Number(f64),
    Str(String),
}

#[derive(Debug)]
pub struct UpValue {
    instack: u8,
    idx: u8,
}

#[derive(Debug)]
pub struct LocVar {
    var_name: String,
    start_pc: u32,
    end_pc: u32,
}

impl BinaryChunk {
    pub fn new() -> BinaryChunk {
        BinaryChunk {
            header: Header {
                signature: [0x1b, b'L', b'u', b'a'],
                version: 0x53,
                format: 0,
                luac_data: [0x19, 0x93, 0x0d, 0x0a, 0x1a, 0x0a],
                c_int_size: 4,
                size_t_size: 8,
                instruction_size: 4,
                lua_integer_size: 8,
                lua_number_size: 8,
                luac_int: 0x5678,
                luac_num: 370.5,
            },
            size_up_values: 0,
            main_func: Prototype::new(),
        }
    }
}

impl Prototype {
    pub fn new() -> Prototype {
        Prototype {
            source: String::new(),
            line_defined: 0,
            last_line_defined: 0,
            num_params: 0,
            is_vararg: 0,
            max_stack_size: 0,
            code: Vec::new(),
            constants: Vec::new(),
            up_values: Vec::new(),
            protos: Vec::new(),
            line_info: Vec::new(),
            loc_vars: Vec::new(),
            up_value_names: Vec::new(),
        }
    }
}

// 解析二进制chunk un_dump
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_un_dump() {
        let data = vec![
            0x1b, b'L', b'u', b'a', 0x53, 0, 0x19, 0x93, 0x0d, 0x0a, 0x1a, 0x0a, 4, 8, 4, 8, 8, 8,
            8, 0x9a, 0x99, 0x19, 0x3f, 0x56, 0x78,
        ];
        let chunk = un_dump(data);
        assert_eq!(chunk.header.signature, [0x1b, b'L', b'u', b'a']);
        assert_eq!(chunk.header.version, 0x53);
        assert_eq!(chunk.header.format, 0);
        assert_eq!(chunk.header.luac_data, [0x19, 0x93, 0x0d, 0x0a, 0x1a, 0x0a]);
        assert_eq!(chunk.header.c_int_size, 4);
        assert_eq!(chunk.header.size_t_size, 8);
        assert_eq!(chunk.header.instruction_size, 4);
        assert_eq!(chunk.header.lua_integer_size, 8);
        assert_eq!(chunk.header.lua_number_size, 8);
        assert_eq!(chunk.size_up_values, 0);
        assert_eq!(chunk.main_func.source, "");
        assert_eq!(chunk.main_func.line_defined, 0);
        assert_eq!(chunk.main_func.last_line_defined, 0);
        assert_eq!(chunk.main_func.num_params, 0);
        assert_eq!(chunk.main_func.is_vararg, 0);
        assert_eq!(chunk.main_func.max_stack_size, 8);
        assert_eq!(chunk.main_func.code, vec![0x9a999193, 0x3f561978]);
        assert_eq!(
            chunk.main_func.constants,
            vec![Constant {
                tag: 0,
                value: ConstantValue::Nil
            }]
        );
        assert_eq!(chunk.main_func.up_values, vec![]);
        assert_eq!(chunk.main_func.protos, vec![]);
        assert_eq!(chunk.main_func.line_info, vec![]);
        assert_eq!(chunk.main_func.loc_vars, vec![]);
        assert_eq!(chunk.main_func.up_value_names, vec![]);
    }

    #[test]
    fn test_un_dump_with_different_data() {
        let data = vec![
            0x1b, b'L', b'u', b'a', 0x53, 0, 0x19, 0x93, 0x0d, 0x0a, 0x1a, 0x0a, 4, 8, 4, 8, 8, 8,
            8, 0x9a, 0x99, 0x19, 0x3f, 0x56, 0x79,
        ];
        let chunk = un_dump(data);
        assert_eq!(chunk.header.signature, [0x1b, b'L', b'u', b'a']);
        assert_eq!(chunk.header.version, 0x53);
        assert_eq!(chunk.header.format, 0);
        assert_eq!(chunk.header.luac_data, [0x19, 0x93, 0x0d, 0x0a, 0x1a, 0x0a]);
        assert_eq!(chunk.header.c_int_size, 4);
        assert_eq!(chunk.header.size_t_size, 8);
        assert_eq!(chunk.header.instruction_size, 4);
        assert_eq!(chunk.header.lua_integer_size, 8);
        assert_eq!(chunk.header.lua_number_size, 8);
        assert_eq!(chunk.size_up_values, 0);
        assert_eq!(chunk.main_func.source, "");
        assert_eq!(chunk.main_func.line_defined, 0);
        assert_eq!(chunk.main_func.last_line_defined, 0);
        assert_eq!(chunk.main_func.num_params, 0);
        assert_eq!(chunk.main_func.is_vararg, 0);
        assert_eq!(chunk.main_func.max_stack_size, 8);
        assert_eq!(chunk.main_func.code, vec![0x9a999193, 0x3f561979]);
        assert_eq!(
            chunk.main_func.constants,
            vec![Constant {
                tag: 0,
                value: ConstantValue::Nil
            }]
        );
        assert_eq!(chunk.main_func.up_values, vec![]);
        assert_eq!(chunk.main_func.protos, vec![]);
        assert_eq!(chunk.main_func.line_info, vec![]);
        assert_eq!(chunk.main_func.loc_vars, vec![]);
        assert_eq!(chunk.main_func.up_value_names, vec![]);

        assert_eq!(chunk.header.size_t_size, 8);
        assert_eq!(chunk.header.instruction_size, 4);
        assert_eq!(chunk.header.lua_integer_size, 8);
        assert_eq!(chunk.header.lua_number_size, 8);
        assert_eq!(chunk.size_up_values, 0);
        assert_eq!(chunk.main_func.source, "");
        assert_eq!(chunk.main_func.line_defined, 0);
        assert_eq!(chunk.main_func.last_line_defined, 0);
        assert_eq!(chunk.main_func.num_params, 0);
        assert_eq!(chunk.main_func.is_vararg, 0);
        assert_eq!(chunk.main_func.max_stack_size, 8);
        assert_eq!(chunk.main_func.code, vec![0x9a999193, 0x3f561978]);
        assert_eq!(
            chunk.main_func.constants,
            vec![Constant {
                tag: 0,
                value: ConstantValue::Nil
            }]
        );
        assert_eq!(chunk.main_func.up_values, vec![]);
        assert_eq!(chunk.main_func.protos, vec![]);
        assert_eq!(chunk.main_func.line_info, vec![]);
        assert_eq!(chunk.main_func.loc_vars, vec![]);
        assert_eq!(chunk.main_func.up_value_names, vec![]);
    }
}
