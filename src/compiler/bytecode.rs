use std::collections::HashMap;

use crate::core::error::{self, ErrorType};

pub struct Bytecode {
    table: HashMap<String, u8>,
}

impl Bytecode {
    pub fn get_handler() -> Bytecode {
        let mut hash_map = HashMap::new();
        hash_map.insert("i64".to_string(), 0x01);
        hash_map.insert("load_const".to_string(), 0x01);
        hash_map.insert("print".to_string(), 0x02);

        Bytecode { table: hash_map }
    }

    pub fn get_bytecode_representation(&mut self, key: String) -> Option<u8> {
        self.table.get(&key).copied()
    }
}

pub fn get_bytecode(item: String) -> u8 {
    let mut bytecode_handler = Bytecode::get_handler();

    if let Some(bytecode) = bytecode_handler.get_bytecode_representation(item) {
        bytecode
    } else {
        error::throw(
            ErrorType::CompilationError,
            "Member name not recognized",
            None,
        );
        std::process::exit(1)
    }
}
