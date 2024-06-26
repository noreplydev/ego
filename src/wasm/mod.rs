use crate::{
    ast::{lex, Module},
    compiler::Compiler,
    core::logs::get_log_history,
    log,
    runtime::Interpreter,
};

pub fn run_ego(code: String, vm: bool) -> Vec<String> {
    log!("Executing ego:");
    log!("  '- code: {}", code);
    log!("  '- vm: {}", vm);
    let tokens = lex(code);
    log!(" > Generated tokens");

    let mut module = Module::new("fileless".to_string(), tokens);
    let ast = module.parse();
    log!(" > Generated ast");
    log!(
        " > Running on {}",
        if vm { "self vm" } else { "Ast interpreter" }
    );
    log!("-------------------");

    if vm {
        let bytecode = Compiler::gen_bytecode(ast);
        let mut vm = self_vm::vm::Vm::new(bytecode);
        vm.run();
        vec!["Logs with executions are not implemented yet".to_string()]
    } else {
        let mut interpreter = Interpreter::new(ast.clone());
        interpreter.exec(false);
        get_log_history()
    }
}
