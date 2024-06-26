use ego::exec_ego_code;

fn main() {
    let code = "print(12)".to_string();
    let use_vm = false;
    exec_ego_code(code, use_vm);
}
