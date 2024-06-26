use crate::{
    ast::{lex, Module},
    compiler::Compiler,
    log,
    runtime::Interpreter,
};

pub fn run_ego(code: String, vm: bool) {
    log!("Executing ego: \n  - code: {}\n  - vm: {}", code, vm);
    let tokens = lex(code);
    log!(" > Generated tokens");

    let mut module = Module::new("fileless".to_string(), tokens);
    let ast = module.parse();
    log!(" > Generated ast");
    log!(
        " > Running on {}",
        if vm { "self vm" } else { "Ast interpreter" }
    );

    if vm {
        let bytecode = Compiler::gen_bytecode(ast);
        let mut vm = self_vm::vm::Vm::new(bytecode);
        vm.run();
    } else {
        let mut interpreter = Interpreter::new(ast.clone());
        interpreter.exec(false);
    }
}
