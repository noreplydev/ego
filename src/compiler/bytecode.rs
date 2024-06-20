use std::collections::HashMap;

pub struct Bytecode {
    table: HashMap<String, u8>,
}

impl Bytecode {
    pub fn get_handler() -> Bytecode {
        let mut hash_map = HashMap::new();
        hash_map.insert("print".to_string(), 0x02);

        Bytecode { table: hash_map }
    }

    pub fn get_bytecode_representation(&mut self, key: String) -> Option<u8> {
        self.table.get(&key).copied()
    }
}
