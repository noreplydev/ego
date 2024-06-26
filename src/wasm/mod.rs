use crate::{
    ast::{lex, Module},
    compiler::Compiler,
    runtime::Interpreter,
};

pub fn run_ego(code: String, vm: bool) {
    let tokens = lex(code);
    let mut module = Module::new("fileless".to_string(), tokens);
    let ast = module.parse();
    if vm {
        let bytecode = Compiler::gen_bytecode(ast);
        let mut vm = self_vm::vm::Vm::new(bytecode);
        vm.run();
    } else {
        let mut interpreter = Interpreter::new(ast.clone());
        interpreter.exec(false);
    }
}
